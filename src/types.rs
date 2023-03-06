use http::Response;
use serde_json::Value;

use crate::clients::Error;

pub type RawResponse = Response<Value>;
pub type Result<T> = std::result::Result<T, Error>;
