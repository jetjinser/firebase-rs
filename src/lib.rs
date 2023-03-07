//! # Quick Start
//!
//! ```rust
//! use std::collections::HashMap;
//! use firebase_rs::{Firebase, Requestable, Paramable};
//! use serde_json::Value;
//!
//! #[tokio::main]
//! async fn run() {
//!     let firebase =
//!            Firebase::new("https://hacker-news.firebaseio.com/v0/").expect("error init Firebase");
//!     let cons = firebase
//!            .at("topstories")
//!            .limit_to_first(7)
//!            .order_by("\"$key\"");
//!     let item_ids = endpoint.get::<Value>().await;
//!
//!     println!("{:?}", item_ids);
//! }
//! ```
//!
//! ## Multi Client Backend
//! The default client is `reqwest`.
//!
//! Available Client:
//! - [reqwest](https://docs.rs/reqwest/latest/reqwest/)
//! - [http_req_wasi](https://github.com/second-state/http_req)
//!
//! ### WASM Support
//! Only Support `wasmedge` now.
//!
//! `Cargo.toml` example:
//! ```toml
//! ...
//! [dependencies]
//! tokio_wasi = { version = "1.25.1", features = ["full"] }
//!
//! [dependencies.firebase-rs]
//! git = "https://github.com/jetjinser/firebase-rs"
//! default-features = false
//! features = ["wasmedge"]
//! ...
//! ```

use clients::{Client, HttpClient};
use constants::AUTH;
use errors::{UrlParseError, UrlParseResult};
use http::{Method, Request};
use serde::Deserialize;
use serde_json::Value;
use std::{fmt::Debug, pin::Pin, sync::Arc};
use tokio::sync::Mutex;
use url::Url;
use utils::check_uri;

pub use http::{Response, Uri};
pub use params::Paramable;
pub use request::Requestable;
pub use types::Result;

mod clients;
mod constants;
mod errors;
mod params;
mod request;
mod types;
mod utils;

/// Represents an instance of Firebase Realtime Database.
#[derive(Debug)]
pub struct Firebase {
    base_uri: Url,
    client: Arc<Mutex<Client>>,
}

impl Firebase {
    /// ```
    /// use firebase_rs::Firebase;
    ///
    /// let firebase = Firebase::new("https://myfirebase.firebaseio.com").unwrap();
    /// ```
    pub fn new(uri: &str) -> UrlParseResult<Self>
    where
        Self: Sized,
    {
        match check_uri(uri) {
            Ok(uri) => Ok(Self {
                base_uri: uri,
                client: Arc::new(Mutex::new(Client::default())),
            }),
            Err(err) => Err(err),
        }
    }

    /// ```
    /// use firebase_rs::Firebase;
    ///
    /// let firebase = Firebase::auth("https://myfirebase.firebaseio.com", "my_auth_key").unwrap();
    /// ```
    pub fn auth(uri: &str, auth_key: &str) -> UrlParseResult<Self>
    where
        Self: Sized,
    {
        match check_uri(uri) {
            Ok(mut uri) => {
                uri.set_query(Some(&format!("{}={}", AUTH, auth_key)));

                Ok(Self {
                    base_uri: uri,
                    client: Arc::new(Mutex::new(Client::default())),
                })
            }
            Err(err) => Err(err),
        }
    }

    /// ```
    /// use firebase_rs::Firebase;
    ///
    /// let uri = "https://myfirebase.firebaseio.com";
    /// let mut firebase = Firebase::new(uri).unwrap();
    /// let endpoint = firebase.at("users");
    ///
    /// let base_uri = firebase.base_uri();
    /// let new_base_uri = endpoint.base_uri();
    ///
    /// assert_eq!(base_uri, format!("{}/", uri));
    /// assert_eq!(new_base_uri, format!("{}/users.json", uri));
    /// ```
    pub fn base_uri(&self) -> String {
        self.base_uri.to_string()
    }
}

impl Firebase {
    /// Returns a new `Firebase` instance with the `base_uri` updated to include the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - A `&str` that represents the path to be added to the base URI.
    ///
    /// # Returns
    ///
    /// A new instance of `Firebase` with the `base_uri` updated to include the given path.
    ///
    /// # Panics
    ///
    /// If the `base_uri` cannot be a base URI.
    pub fn at(&self, path: &str) -> Self {
        let re_path: String = self
            .base_uri
            .path_segments()
            .unwrap_or_else(|| panic!("cannot be base"))
            .map(|seg| format!("{}/", seg.trim_end_matches(".json")))
            .collect();

        let new_path = re_path + path;

        let mut uri = self.base_uri.clone();
        uri.set_path(&format!("{}.json", new_path.trim_end_matches(".json")));

        Firebase {
            base_uri: uri,
            client: Arc::clone(&self.client),
        }
    }
}

impl Requestable for Firebase {
    fn request<'life0, 'async_trait, Resp>(
        &'life0 self,
        method: Method,
        data: Option<Value>,
    ) -> Pin<
        Box<
            dyn core::future::Future<Output = Result<Response<Resp>>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        Resp: for<'a> Deserialize<'a>,
        Resp: 'async_trait,
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            let req = Request::builder()
                .method(method)
                .uri(
                    self.base_uri
                        .to_string()
                        .parse::<Uri>()
                        .expect("infallible"),
                )
                .body(data)
                .unwrap();
            let client = self.client.lock().await;
            (*client).send(req).await
        })
    }
}

impl Paramable for Firebase {
    fn add_param<T>(&self, key: &str, value: T) -> Self
    where
        T: ToString,
    {
        let mut uri = self.base_uri.clone();
        uri.query_pairs_mut().append_pair(key, &value.to_string());

        Self {
            base_uri: uri,
            client: Arc::clone(&self.client),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Firebase, UrlParseError};

    const URI: &str = "https://firebase_id.firebaseio.com";
    const URI_WITH_SLASH: &str = "https://firebase_id.firebaseio.com/";
    const URI_NON_HTTPS: &str = "http://firebase_id.firebaseio.com/";

    #[tokio::test]
    async fn simple() {
        let firebase = Firebase::new(URI).unwrap();
        assert_eq!(URI_WITH_SLASH.to_string(), firebase.base_uri());
    }

    #[tokio::test]
    async fn non_https() {
        let firebase = Firebase::new(URI_NON_HTTPS).map_err(|e| e.to_string());
        assert_eq!(
            firebase.err(),
            Some(String::from(UrlParseError::NotHttps.to_string()))
        );
    }

    #[tokio::test]
    async fn with_auth() {
        let firebase = Firebase::auth(URI, "auth_key").unwrap();
        assert_eq!(
            format!("{}/?auth=auth_key", URI.to_string()),
            firebase.base_uri()
        );
    }
}
