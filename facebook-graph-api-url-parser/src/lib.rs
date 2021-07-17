use http::uri::{InvalidUri as HttpInvalidUri, Uri};
use url::{ParseError as UrlParseError, Url};

const SCHEME: &str = "https";
const HOST: &str = "graph.facebook.com";

pub fn parse(url: &str) -> Result<(Version), ParseError> {
    let uri: Uri = url.parse()?;
    if let Some(scheme_str) = uri.scheme_str() {
        if scheme_str != SCHEME {
            return Err(ParseError::UrlSchemeMismatch);
        }
    }
    if let Some(host) = uri.host() {
        if host != HOST {
            return Err(ParseError::UrlHostMismatch);
        }
    }

    let url = format!(
        "{}://{}{}",
        uri.scheme_str().unwrap_or(SCHEME),
        uri.host().unwrap_or(HOST),
        uri.path_and_query().map(|x| x.as_str()).unwrap_or("/")
    );
    let url = Url::parse(&url)?;
    let mut path_segments = url.path_segments().ok_or(ParseError::UrlPathInvalid)?;

    let version = path_segments.next().ok_or(ParseError::VersionMissing)?;
    let version = match version {
        "v11.0" => Version::V11,
        "v10.0" => Version::V10,
        _ => return Err(ParseError::VersionMismatch),
    };

    Ok((version))
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Version {
    V11,
    V10,
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
    #[error("UrlPathInvalid")]
    UrlPathInvalid,
    #[error("VersionMissing")]
    VersionMissing,
    #[error("VersionMismatch")]
    VersionMismatch,
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

        match parse("https://graph.facebook.com") {
            Err(ParseError::VersionMismatch) => {}
            err => assert!(false, "{:?}", err),
        }
    }
}
