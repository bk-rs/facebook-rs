use url::{ParseError as UrlParseError, Url};

const PREFIX: &str = "https://graph.facebook.com/v11.0";

pub fn parse(path_and_query: &str) -> Result<(), ParseError> {
    let url = if path_and_query.is_empty() {
        PREFIX.to_string()
    } else if path_and_query.starts_with('/') {
        format!("{}{}", PREFIX, path_and_query)
    } else {
        format!("{}/{}", PREFIX, path_and_query)
    };
    let url = Url::parse(&url)?;
    let mut path_segments = url.path_segments().expect("");
    debug_assert_eq!(path_segments.next(), Some("v11.0"));

    let root = path_segments.next().ok_or(ParseError::RootMissing)?;

    if let Ok(_node_id) = root.parse::<u64>() {
    } else {
        match root {
            "me" => {}
            _ => return Err(ParseError::RootIsUnknown),
        }
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("PathOrQueryInvalid {0}")]
    PathOrQueryInvalid(#[from] UrlParseError),
    #[error("RootMissing")]
    RootMissing,
    #[error("RootIsUnknown")]
    RootIsUnknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        match parse("") {
            Err(ParseError::RootMissing) => {}
            err => assert!(false, "{:?}", err),
        }

        match parse("foo") {
            Err(ParseError::RootIsUnknown) => {}
            err => assert!(false, "{:?}", err),
        }
    }
}
