use url::{ParseError as UrlParseError, Url};

const PREFIX: &str = "https://graph.facebook.com/v11.0";

pub fn parse<'a>(path_and_query: &str, node_type: Option<&'a str>) -> Result<Root<'a>, ParseError> {
    let url = if path_and_query.is_empty() {
        return Err(ParseError::PathInvalid("IsEmpty"));
    } else if path_and_query.starts_with('/') {
        return Err(ParseError::PathInvalid("IsStartsWithSlash"));
    } else {
        format!("{}/{}", PREFIX, path_and_query)
    };
    let mut url = Url::parse(&url)?;
    let path = url.path().to_owned();
    if path.ends_with('/') {
        url.set_path(&path[..path.len() - 1]);
    }
    let mut path_segments = url.path_segments().expect("");
    debug_assert_eq!(path_segments.next(), Some("v11.0"));
    let mut _query_pairs = url.query_pairs();

    let root = path_segments.next().ok_or(ParseError::RootMissing)?;
    let mut root = if let Ok(_node_id) = root.parse::<u64>() {
        let node_type = node_type.ok_or(ParseError::NodeTypeMissing)?;
        Root::Node(node_type, None)
    } else {
        match root {
            "me" => Root::Node("User", None),
            "ig_hashtag_search" => Root::Edge(root.to_owned()),
            _ => return Err(ParseError::RootIsUnknown),
        }
    };
    match root {
        Root::Node(node_type, _) => {
            if let Some(edge) = path_segments.next() {
                root = Root::Node(node_type, Some(edge.to_owned()))
            }
            if path_segments.next().is_some() {
                return Err(ParseError::PathInvalid("TODO"));
            }
        }
        Root::Edge(_) => {
            if path_segments.next().is_some() {
                return Err(ParseError::PathInvalid("TODO"));
            }
        }
    }

    Ok(root)
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Root<'a> {
    Node(&'a str, Option<String>),
    Edge(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("PathInvalid {0}")]
    PathInvalid(&'static str),
    #[error("PathOrQueryInvalid {0}")]
    PathOrQueryInvalid(#[from] UrlParseError),
    #[error("RootMissing")]
    RootMissing,
    #[error("RootIsUnknown")]
    RootIsUnknown,
    #[error("NodeTypeMissing")]
    NodeTypeMissing,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        match parse("", None) {
            Err(ParseError::PathInvalid(_)) => {}
            err => panic!("{:?}", err),
        }
        match parse("/", None) {
            Err(ParseError::PathInvalid(_)) => {}
            err => panic!("{:?}", err),
        }

        match parse("foo", None) {
            Err(ParseError::RootIsUnknown) => {}
            err => panic!("{:?}", err),
        }

        match parse("me", None) {
            Ok(root) => assert_eq!(root, Root::Node("User", None)),
            err => panic!("{:?}", err),
        }
        match parse("780170842505209", Some("User")) {
            Ok(root) => assert_eq!(root, Root::Node("User", None)),
            err => panic!("{:?}", err),
        }
        match parse("me?fields=id,name", None) {
            Ok(root) => assert_eq!(root, Root::Node("User", None)),
            err => panic!("{:?}", err),
        }
        match parse("me/accounts", None) {
            Ok(root) => assert_eq!(root, Root::Node("User", Some("accounts".to_owned()))),
            err => panic!("{:?}", err),
        }
        match parse("me/accounts?fields=id,name", None) {
            Ok(root) => assert_eq!(root, Root::Node("User", Some("accounts".to_owned()))),
            err => panic!("{:?}", err),
        }
        match parse("me/accounts/foo", None) {
            Err(ParseError::PathInvalid(_)) => {}
            err => panic!("{:?}", err),
        }

        match parse(
            "ig_hashtag_search?user_id=17841406427775093&q=bluebottle",
            None,
        ) {
            Ok(root) => assert_eq!(root, Root::Edge("ig_hashtag_search".to_owned())),
            err => panic!("{:?}", err),
        }
        match parse("ig_hashtag_search/foo", None) {
            Err(ParseError::PathInvalid(_)) => {}
            err => panic!("{:?}", err),
        }
    }
}
