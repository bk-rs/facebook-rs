use http::uri::{InvalidUri as HttpInvalidUri, Uri};
use url::{ParseError as UrlParseError, Url};

pub fn parse(url: &str) -> Result<(), ParseError> {
    let uri: Uri = url.parse()?;
    if let Some(scheme_str) = uri.scheme_str() {
        if scheme_str != "https" {
            return Err(ParseError::UrlSchemeMismatch);
        }
    }
    if let Some(host) = uri.host() {
        if host != "graph.facebook.com" {
            return Err(ParseError::UrlHostMismatch);
        }
    }

    let _ = Url::parse(url)?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("HttpInvalidUri {0}")]
    HttpInvalidUri(#[from] HttpInvalidUri),
    #[error("UrlSchemeMismatch")]
    UrlSchemeMismatch,
    #[error("UrlHostMismatch")]
    UrlHostMismatch,
    #[error("UrlParseError {0}")]
    UrlParseError(#[from] UrlParseError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        match parse("http://graph.facebook.com/v11.0/me") {
            Err(ParseError::UrlSchemeMismatch) => {}
            err => assert!(false, "{:?}", err),
        }

        match parse("https://www.facebook.com/v11.0/me") {
            Err(ParseError::UrlHostMismatch) => {}
            err => assert!(false, "{:?}", err),
        }
    }
}
