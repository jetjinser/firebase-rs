use clients::{Client, Error, HttpClient};
use constants::AUTH;
use errors::{UrlParseError, UrlParseResult};
use http::{Method, Request};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use url::Url;
use utils::check_uri;

pub use http::{Response, Uri};
pub use params::Paramable;

pub type RawResponse = Response<Value>;

mod clients;
mod constants;
mod errors;
mod params;
mod utils;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Firebase {
    uri: Url,
    client: Client,
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
                uri,
                client: Client::default(),
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
                    uri,
                    client: Client::default(),
                })
            }
            Err(err) => Err(err),
        }
    }

    /// ```
    /// use firebase_rs::Firebase;
    ///
    /// let firebase = Firebase::new("https://myfirebase.firebaseio.com").unwrap().at("users").at("USER_ID").at("f69111a8a5258c15286d3d0bd4688c55");
    /// ```
    pub fn at(&mut self, path: &str) -> &mut Self {
        let re_path: String = self
            .uri
            .path_segments()
            .unwrap_or_else(|| panic!("cannot be base"))
            .map(|seg| format!("{}/", seg.trim_end_matches(".json")))
            .collect();

        let new_path = re_path + path;

        self.uri
            .set_path(&format!("{}.json", new_path.trim_end_matches(".json")));

        self
    }

    /// ```
    /// use firebase_rs::Firebase;
    ///
    /// let mut firebase = Firebase::new("https://myfirebase.firebaseio.com").unwrap();
    /// let endpoint = firebase.at("users");
    /// let uri = endpoint.get_uri();
    /// ```
    pub fn get_uri(&self) -> String {
        self.uri.to_string()
    }

    async fn request<Resp>(&self, method: Method, data: Option<Value>) -> Result<Response<Resp>>
    where
        Resp: for<'a> Deserialize<'a>,
    {
        let req = Request::builder()
            .method(method)
            .uri(&self.uri.to_string().parse::<Uri>().expect("infallible"))
            .body(data)
            .unwrap();

        self.client.send(req).await
    }

    async fn request_generic<T>(&self, method: Method) -> Result<T>
    where
        T: Serialize + DeserializeOwned + Debug,
    {
        let request = self.request::<T>(method, None).await;

        match request {
            Ok(response) => Ok(response.into_body()),
            Err(err) => Err(err),
        }
    }

    /// ```
    /// use firebase_rs::{Firebase, Response};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct User {
    ///    name: String
    /// }
    ///
    /// # async fn run() {
    /// let user = User { name: String::default() };
    /// let mut firebase = Firebase::new("https://myfirebase.firebaseio.com").unwrap();
    /// let endpoint = firebase.at("users");
    /// let users: Result<Response<String>, _> = endpoint.set(&user).await;
    /// # }
    /// ```
    pub async fn set<T, Resp>(&self, data: &T) -> Result<Response<Resp>>
    where
        T: Serialize + DeserializeOwned + Debug,
        Resp: for<'a> Deserialize<'a>,
    {
        let data = serde_json::to_value(data).unwrap();
        self.request(Method::POST, Some(data)).await
    }

    /// ```
    /// use std::collections::HashMap;
    /// use firebase_rs::Firebase;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct User {
    ///    name: String
    /// }
    ///
    /// # async fn run() {
    /// let mut firebase = Firebase::new("https://myfirebase.firebaseio.com").unwrap();
    /// let endpoint = firebase.at("users");
    /// let users = endpoint.get::<HashMap<String, User>>().await;
    /// # }
    /// ```
    pub async fn get_as_string(&self) -> Result<Response<String>> {
        self.request(Method::GET, None).await
    }

    /// ```
    /// use std::collections::HashMap;
    /// use firebase_rs::Firebase;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct User {
    ///     name: String
    /// }
    ///
    /// # async fn run() {
    /// let mut firebase = Firebase::new("https://myfirebase.firebaseio.com").unwrap();
    /// let endpoint = firebase.at("users").at("USER_ID");
    /// let user = endpoint.get::<User>().await;
    ///
    ///  // OR
    ///
    /// let mut firebase = Firebase::new("https://myfirebase.firebaseio.com").unwrap();
    /// let endpoint = firebase.at("users");
    /// let user = endpoint.get::<HashMap<String, User>>().await;
    /// # }
    /// ```
    pub async fn get<T>(&self) -> Result<T>
    where
        T: Serialize + DeserializeOwned + Debug,
    {
        self.request_generic::<T>(Method::GET).await
    }

    /// ```
    /// use firebase_rs::{Firebase, Response, Result};
    ///
    /// # async fn run() {
    /// let mut firebase = Firebase::new("https://myfirebase.firebaseio.com").unwrap();
    /// let endpoint = firebase.at("users").at("USER_ID");
    /// endpoint.delete::<serde_json::Value>().await;
    /// # }
    /// ```
    pub async fn delete<Resp>(&self) -> Result<Response<Resp>>
    where
        Resp: for<'a> Deserialize<'a>,
    {
        self.request(Method::DELETE, None).await
    }

    /// ```
    /// use firebase_rs::{Firebase, Response};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct User {
    ///     name: String
    /// }
    ///
    /// # async fn run() {
    /// let user = User { name: String::default() };
    /// let mut firebase = Firebase::new("https://myfirebase.firebaseio.com").unwrap();
    /// let endpoint = firebase.at("users").at("USER_ID");
    /// let users: Response<serde_json::Value> = endpoint.update(&user).await.unwrap();
    /// # }
    /// ```
    pub async fn update<T, Resp>(&self, data: &T) -> Result<Response<Resp>>
    where
        T: DeserializeOwned + Serialize + Debug,
        Resp: for<'a> Deserialize<'a>,
    {
        let value = serde_json::to_value(data).unwrap();
        self.request(Method::PATCH, Some(value)).await
    }
}

impl Paramable for Firebase {
    fn add_param<T>(&mut self, key: &str, value: T) -> &mut Self
    where
        T: ToString,
    {
        self.uri
            .query_pairs_mut()
            .append_pair(key, &value.to_string());

        self
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
        assert_eq!(URI_WITH_SLASH.to_string(), firebase.get_uri());
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
            firebase.get_uri()
        );
    }
}
