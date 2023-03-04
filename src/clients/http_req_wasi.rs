use std::{future::Future, pin::Pin, str::FromStr};

use http::{HeaderName, HeaderValue, Method, Request, Response, Version};
use http_req::{
    request::{HttpVersion, Method as ReqMethod, RequestBuilder},
    response::Headers,
    uri::Uri,
};
use serde::Deserialize;
use serde_json::Value;

use super::HttpClient;

#[derive(Debug, Default)]
pub struct Client;

pub type Error = http_req::error::Error;

impl<T> HttpClient<T> for Client
where
    T: for<'a> Deserialize<'a>,
{
    type Error = Error;

    fn send<'life0, 'async_trait>(
        &'life0 self,
        req: Request<Option<Value>>,
    ) -> Pin<Box<dyn Future<Output = Result<Response<T>, Self::Error>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let http_uri = req.uri().to_string();
            let uri = Uri::try_from(http_uri.as_str()).expect("infallible");

            let method = req.method();
            let method = if method == Method::GET {
                ReqMethod::GET
            } else if method == Method::POST {
                ReqMethod::POST
            } else if method == Method::PUT {
                ReqMethod::PUT
            } else if method == Method::DELETE {
                ReqMethod::DELETE
            } else if method == Method::HEAD {
                ReqMethod::HEAD
            } else if method == Method::OPTIONS {
                ReqMethod::OPTIONS
            } else {
                panic!("unspported method")
            };

            let mut headers = Headers::default_http(&uri);

            headers.insert("User-Agent", "Rusted Firebase");
            for (k, v) in req.headers() {
                headers.insert(k, v.to_str().unwrap_or(""));
            }

            let version = req.version();
            let version = if version == Version::HTTP_09 {
                panic!("unspported http version")
            } else if version == Version::HTTP_10 {
                HttpVersion::Http10
            } else if version == Version::HTTP_11 {
                HttpVersion::Http11
            } else if version == Version::HTTP_2 {
                HttpVersion::Http20
            } else if version == Version::HTTP_3 {
                panic!("unspported http version")
            } else {
                panic!("unspported http version")
            };

            let mut writer = Vec::new();
            let response = RequestBuilder::new(&uri)
                .method(method)
                .headers(headers)
                .version(version)
                .send(&mut writer);

            // TODO: body...

            // let request = if let Some(body) = req.body() {
            //     let bs = body.to_string();
            //     request.body(bs.as_bytes())
            // } else {
            //     request
            // };

            match response {
                Ok(resp) => {
                    let mut resp_builder = Response::builder()
                        .status(u16::from(resp.status_code()))
                        .version(to_version(resp.version()));

                    let headers = resp_builder.headers_mut().unwrap();

                    for (k, v) in resp.headers().iter() {
                        let k = k.to_owned().into_inner();
                        headers.append(
                            // wtf..
                            HeaderName::from_str(k.as_str()).unwrap(),
                            HeaderValue::from_str(v).unwrap(),
                        );
                    }

                    let body = serde_json::from_slice::<T>(&writer)
                        .map_err(|e| format!("e: {}\nraw: {}", e, String::from_utf8_lossy(&writer)))
                        .unwrap();

                    Ok(resp_builder.body(body).unwrap())
                }
                Err(e) => Err(e),
            }
        })
    }
}

#[inline]
fn to_version(v: &str) -> Version {
    match v {
        "HTTP/0.9" => Version::HTTP_09,
        "HTTP/1.0" => Version::HTTP_10,
        "HTTP/1.1" => Version::HTTP_11,
        "HTTP/2.0" => Version::HTTP_2,
        "HTTP/3.0" => Version::HTTP_3,
        _ => panic!("unspported version"),
    }
}
