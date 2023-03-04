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

#[async_trait]
pub trait HttpClient<T>
where
    T: for<'a> Deserialize<'a>,
{
    type Error: std::error::Error + Send + Sync + 'static;

    async fn send(&self, req: Request<Option<Value>>) -> Result<Response<T>, Self::Error>;
}
