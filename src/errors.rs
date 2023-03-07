use std::fmt::{Display, Formatter};

/// Type alias for `Result<T, UrlParseError>` type.
pub type UrlParseResult<T> = Result<T, UrlParseError>;

/// Error enum for URL parsing errors.
#[derive(Debug)]
pub enum UrlParseError {
    /// The URL path is missing.
    NoPath,
    /// The URL protocol should be HTTPS.
    NotHttps,
    /// Error occurred while parsing the URL.
    Parser(url::ParseError),
}

impl Display for UrlParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlParseError::NoPath => write!(f, "URL path is missing."),
            UrlParseError::NotHttps => write!(f, "The URL protocol should be https."),
            UrlParseError::Parser(e) => write!(f, "Error while parsing the URL: {}", e),
        }
    }
}
