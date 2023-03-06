use crate::types::Result;
use async_trait::async_trait;

use http::{Method, Response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;

#[async_trait]
pub trait Requestable<'fb> {
    async fn request<Resp>(&self, method: Method, data: Option<Value>) -> Result<Response<Resp>>
    where
        Resp: for<'a> Deserialize<'a>;

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
    async fn set<T, Resp>(&self, data: &T) -> Result<Response<Resp>>
    where
        T: Serialize + DeserializeOwned + Debug + Send + Sync,
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
    async fn get_as_string(&self) -> Result<Response<String>> {
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
    async fn get<T>(&self) -> Result<T>
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
    async fn delete<Resp>(&self) -> Result<Response<Resp>>
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
    async fn update<T, Resp>(&self, data: &T) -> Result<Response<Resp>>
    where
        T: DeserializeOwned + Serialize + Debug + Send + Sync,
        Resp: for<'a> Deserialize<'a>,
    {
        let value = serde_json::to_value(data).unwrap();
        self.request(Method::PATCH, Some(value)).await
    }
}
