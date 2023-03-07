use async_trait::async_trait;
use http::{Request, Response};
use serde::Deserialize;
use serde_json::Value;

#[cfg(feature = "reqwest")]
mod reqwest;

#[cfg(feature = "reqwest")]
pub use self::reqwest::{Client, Error};

// ---

#[cfg(feature = "http_req_wasi")]
mod http_req_wasi;

#[cfg(feature = "http_req_wasi")]
pub use self::http_req_wasi::{Client, Error};

// ---

/// HTTP Client trait for sending HTTP requests and receiving responses.
#[async_trait]
pub trait HttpClient<T>
where
    T: for<'a> Deserialize<'a>,
{
    /// The error type returned by the `send` method.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Sends an HTTP request and returns an HTTP response.
    ///
    /// # Arguments
    ///
    /// * `req` - An `http::Request` object with an optional `serde_json::Value` body.
    ///
    /// # Returns
    ///
    /// An `http::Response` object with the deserialized `T` body or an `Error`.
    async fn send(&self, req: Request<Option<Value>>) -> Result<Response<T>, Self::Error>;
}
