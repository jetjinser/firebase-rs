use crate::errors::UrlParseResult;
use crate::UrlParseError;
use http::{Method, Request, Uri};
use serde_json::Value;
use url::Url;

pub fn check_uri(uri: &str) -> UrlParseResult<Url> {
    let uri = uri.trim_end_matches('/').parse::<Url>();

    let uri = match uri {
        Ok(res) => res,
        Err(err) => return Err(UrlParseError::Parser(err)),
    };

    if uri.scheme() != "https" {
        return Err(UrlParseError::NotHttps);
    }

    Ok(uri)
}

pub(crate) fn make_request(
    uri: &Url,
    method: Method,
    data: Option<Value>,
) -> Request<Option<Value>> {
    Request::builder()
        .method(method)
        .uri(uri.to_string().parse::<Uri>().expect("infallible"))
        .body(data)
        .unwrap()
}
