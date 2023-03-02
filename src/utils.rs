use url::Url;

use crate::errors::UrlParseResult;
use crate::UrlParseError;

pub fn check_uri(uri: &str) -> UrlParseResult<Url> {
    let uri = uri.trim_end_matches("/").parse::<Url>();

    let uri = match uri {
        Ok(res) => res,
        Err(err) => return Err(UrlParseError::Parser(err)),
    };

    if uri.scheme() != "https" {
        return Err(UrlParseError::NotHttps);
    }

    Ok(uri)
}
