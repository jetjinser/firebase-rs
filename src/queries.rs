use http::{Method, Response};
use serde::Deserialize;
use serde_json::Value;
use url::Url;

use crate::{
    clients::HttpClient, params::WithParams, types, utils::make_request, Firebase, Requestable,
};

pub struct WithQueries<'fb> {
    pub firebase: &'fb Firebase,
    pub uri: Url,
}

impl<'fb> WithQueries<'fb> {
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

    pub fn finish(self) -> &'fb Firebase {
        self.firebase
    }
}

impl Requestable<'_> for WithQueries<'_> {
    fn request<'life0, 'async_trait, Resp>(
        &'life0 self,
        method: Method,
        data: Option<Value>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = types::Result<Response<Resp>>>
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
            let req = make_request(&self.uri, method, data);

            self.firebase.client.send(req).await
        })
    }
}

impl WithQueries<'_> {
    pub fn with_params(&self) -> WithParams {
        WithParams {
            firebase: self.firebase,
            uri: self.uri.clone(),
        }
    }
}
