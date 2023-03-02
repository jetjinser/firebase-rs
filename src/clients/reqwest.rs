use std::{future::Future, pin::Pin};

use http::{Request, Response};
use reqwest::Client as ReqClient;
use serde::Deserialize;
use serde_json::Value;

use super::HttpClient;

#[derive(Debug, Default)]
pub struct Client {
    inner: ReqClient,
}

pub type Error = reqwest::Error;

impl<T> HttpClient<T> for Client
where
    T: for<'a> Deserialize<'a>,
{
    type Error = Error;

    fn send<'life0, 'async_trait>(
        &'life0 self,
        req: Request<Option<Value>>,
    ) -> Pin<Box<(dyn Future<Output = Result<Response<T>, Self::Error>> + Send + 'async_trait)>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let request = self
                .inner
                .request(req.method().to_owned(), req.uri().to_string())
                .headers(req.headers().to_owned())
                .version(req.version())
                .json(req.body());

            let response = request.send().await;

            match response {
                Ok(resp) => {
                    let mut resp_builder = Response::builder()
                        // .extension(resp.extensions())
                        .status(resp.status())
                        .version(resp.version());

                    let headers = resp_builder.headers_mut().unwrap();

                    for (k, v) in resp.headers() {
                        headers.append(k, v.into());
                    }

                    Ok(resp_builder.body(resp.json::<T>().await.unwrap()).unwrap())
                }
                Err(e) => Err(e),
            }
        })
    }
}
