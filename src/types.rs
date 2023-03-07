use crate::clients::Error;

/// Result with Client Error
pub type Result<T> = std::result::Result<T, Error>;
