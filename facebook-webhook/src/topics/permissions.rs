//! [Official doc](https://developers.facebook.com/docs/graph-api/webhooks/reference/permissions/)
//!
//! Don't require [Page Subscribed Apps](https://developers.facebook.com/docs/graph-api/reference/page/subscribed_apps#Creating)

use serde::{de::Deserializer, Deserialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "field", rename_all = "snake_case")]
pub enum Permissions {
    Connected(FieldFlattenValue),
    InstagramBasic(FieldValue),
    InstagramManageComments(FieldValue),
    InstagramManageInsights(FieldValue),
    InstagramContentPublish(FieldValue),
    PagesShowList(FieldValue),
    PagesManageMetadata(FieldValue),
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Verb {
    Granted,
    Revoked,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FieldFlattenValue {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FieldValue {
    pub value: Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Value {
    pub verb: Verb,
    #[serde(default, deserialize_with = "deserialize_target_ids")]
    pub target_ids: Option<Vec<u64>>,
}

//
fn deserialize_target_ids<'de, D>(deserializer: D) -> Result<Option<Vec<u64>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum VecOrNull {
        Vec(Vec<String>),
        Null,
    }

    match VecOrNull::deserialize(deserializer)? {
        VecOrNull::Vec(v) => v
            .into_iter()
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map(Some)
            .map_err(serde::de::Error::custom),
        VecOrNull::Null => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = r#"
        {
            "field": "connected",
            "verb": "granted"
        }
        "#;
        match serde_json::from_str::<Permissions>(json) {
            Ok(Permissions::Connected(v)) => {
                println!("{:?}", v);

                assert_eq!(v.value.verb, Verb::Granted);
                assert_eq!(v.value.target_ids, None);
            }
            Ok(v) => panic!("{:?}", v),
            Err(err) => panic!("{}", err),
        }

        let json = r#"
        {
            "field": "instagram_basic",
            "value": {
                "verb": "granted",
                "target_ids": [
                    "123123123123123",
                    "321321321321321"
                ]
            }
        }
        "#;
        match serde_json::from_str::<Permissions>(json) {
            Ok(Permissions::InstagramBasic(v)) => {
                println!("{:?}", v);

                assert_eq!(v.value.verb, Verb::Granted);
                assert_eq!(
                    v.value.target_ids,
                    Some(vec![123123123123123, 321321321321321])
                );
            }
            Ok(v) => panic!("{:?}", v),
            Err(err) => panic!("{}", err),
        }
    }
}
