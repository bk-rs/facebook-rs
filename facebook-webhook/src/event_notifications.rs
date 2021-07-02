//! [Official doc](https://developers.facebook.com/docs/graph-api/webhooks/getting-started#event-notifications)

use std::{
    error,
    future::Future,
    pin::Pin,
    str::{self, FromStr},
    sync::Arc,
};

use chrono::{serde::ts_seconds, DateTime, Utc};
use hmac::{Hmac, Mac as _, NewMac as _};
use http::StatusCode;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sha1::Sha1;

use crate::topics::{instagram::Instagram, permissions::Permissions};

type HmacSha1 = Hmac<Sha1>;

pub const SIGNATURE_HEADER_NAME: &str = "X-Hub-Signature";

#[derive(PartialEq, Debug, Clone)]
pub enum Signature {
    Sha1(String),
}

impl FromStr for Signature {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('=');

        let algorithm = split.next().ok_or("algorithm missing")?;
        let value = split.next().ok_or("value missing")?;
        if split.next().is_some() {
            return Err("header invalid");
        }

        match algorithm {
            "sha1" => {
                if value.len() != 40 {
                    return Err("value length invalid");
                }

                Ok(Self::Sha1(value.to_owned()))
            }
            _ => Err("algorithm unknown"),
        }
    }
}

pub fn verify_payload(
    signature_header_value: &[u8],
    request_body_bytes: &[u8],
    app_secret: &str,
) -> Result<(), VerifyPayloadError> {
    let signature_header_value = str::from_utf8(signature_header_value)
        .map_err(|_| VerifyPayloadError::SignatureHeaderValueInvalid("header invalid"))?;

    let signature = signature_header_value
        .parse()
        .map_err(VerifyPayloadError::SignatureHeaderValueInvalid)?;

    match signature {
        Signature::Sha1(expected_sig) => {
            let sig = sha1_payload(request_body_bytes, app_secret)
                .map_err(|_| VerifyPayloadError::CalculateSignatureFailed)?;

            if expected_sig.to_ascii_lowercase() != sig {
                return Err(VerifyPayloadError::SignatureMismatch);
            }
        }
    }

    Ok(())
}

// $ echo -n "value" | openssl sha1 -hmac "key"
// (stdin)= 57443a4c052350a44638835d64fd66822f813319
fn sha1_payload(request_body_bytes: &[u8], app_secret: &str) -> Result<String, String> {
    let mut hmac =
        HmacSha1::new_from_slice(app_secret.as_bytes()).map_err(|err| err.to_string())?;
    hmac.update(request_body_bytes);
    let hmac_result = hmac.finalize().into_bytes();
    let sig = hex::encode(hmac_result).to_ascii_lowercase();

    Ok(sig)
}

#[derive(thiserror::Error, Debug)]
pub enum VerifyPayloadError {
    #[error("SignatureHeaderValueInvalid")]
    SignatureHeaderValueInvalid(&'static str),
    #[error("CalculateSignatureFailed")]
    CalculateSignatureFailed,
    #[error("SignatureMismatch")]
    SignatureMismatch,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "object", content = "entry", rename_all = "snake_case")]
pub enum Payload {
    Instagram(Vec<InstagramObjectEntry>),
    Permissions(Vec<PermissionsObjectEntry>),
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstagramObjectEntry {
    /// id == [IG User id](https://developers.facebook.com/docs/instagram-api/reference/ig-user)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    #[serde(with = "ts_seconds")]
    pub time: DateTime<Utc>,
    pub changes: Vec<Instagram>,
}
impl InstagramObjectEntry {
    pub fn is_test(&self) -> bool {
        self.id == 0
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PermissionsObjectEntry {
    /// id == uid == [FB Business Integration User ID](https://www.facebook.com/settings?tab=business_tools&ref=settings)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub uid: u64,
    #[serde(with = "ts_seconds")]
    pub time: DateTime<Utc>,
    pub changes: Vec<Permissions>,
}
impl PermissionsObjectEntry {
    pub fn is_test(&self) -> bool {
        self.id == 0
    }
}

pub type PassBackCallbackFn<'a, C> = Box<
    dyn Fn(Payload, C) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn error::Error>>> + Send>>
        + Send
        + Sync
        + 'a,
>;

pub async fn pass_back<C>(
    signature_header_value: &[u8],
    request_body_bytes: &[u8],
    app_secret: &str,
    ctx: C,
    callback: Arc<PassBackCallbackFn<'_, C>>,
) -> PassBackResponse {
    match verify_payload(signature_header_value, request_body_bytes, app_secret) {
        Ok(_) => match serde_json::from_slice::<Payload>(request_body_bytes) {
            Ok(payload) => match callback(payload, ctx).await {
                Ok(_) => PassBackResponse {
                    status_code: StatusCode::OK,
                    body: "".to_owned(),
                },
                Err(err) => PassBackResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    body: err.to_string(),
                },
            },
            Err(err) => PassBackResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                body: err.to_string(),
            },
        },
        Err(err) => match err {
            VerifyPayloadError::SignatureHeaderValueInvalid(_) => PassBackResponse {
                status_code: StatusCode::BAD_REQUEST,
                body: err.to_string(),
            },
            VerifyPayloadError::CalculateSignatureFailed
            | VerifyPayloadError::SignatureMismatch => PassBackResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                body: err.to_string(),
            },
        },
    }
}

#[derive(Debug, Clone)]
pub struct PassBackResponse {
    pub status_code: StatusCode,
    pub body: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_signature() {
        assert_eq!(
            "sha1=57443a4c052350a44638835d64fd66822f813319"
                .parse::<Signature>()
                .unwrap(),
            Signature::Sha1("57443a4c052350a44638835d64fd66822f813319".to_owned())
        );
    }

    #[test]
    fn test_sha1_payload() {
        assert_eq!(
            sha1_payload(b"value", "key").unwrap(),
            "57443a4c052350a44638835d64fd66822f813319"
        );
    }

    #[test]
    fn test_payload() {
        let json = r#"
        {
            "object": "instagram",
            "entry": [
                {
                    "id": "0",
                    "time": 1624005617,
                    "changes": [
                        {
                            "field": "story_insights",
                            "value": {
                                "media_id": "17887498072083520",
                                "impressions": 444,
                                "reach": 44,
                                "taps_forward": 4,
                                "taps_back": 3,
                                "exits": 3,
                                "replies": 0
                            }
                        }
                    ]
                }
            ]
        }
        "#;

        match serde_json::from_str::<Payload>(json) {
            Ok(Payload::Instagram(entry_vec)) => {
                println!("{:?}", entry_vec);

                assert_eq!(entry_vec.len(), 1);
                let entry = entry_vec.first().unwrap();
                assert_eq!(entry.id, 0);
                assert_eq!(entry.changes.len(), 1);
            }
            Ok(payload) => assert!(false, "{:?}", payload),
            Err(err) => assert!(false, "{}", err),
        }

        let json = r#"
        {
            "object": "permissions",
            "entry": [
                {
                    "id": "0",
                    "uid": "0",
                    "time": 1624610156,
                    "changes": [
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
                    ]
                }
            ]
        }
        "#;
        match serde_json::from_str::<Payload>(json) {
            Ok(Payload::Permissions(entry_vec)) => {
                println!("{:?}", entry_vec);

                assert_eq!(entry_vec.len(), 1);
                let entry = entry_vec.first().unwrap();
                assert_eq!(entry.id, 0);
                assert_eq!(entry.changes.len(), 1);
            }
            Ok(payload) => assert!(false, "{:?}", payload),
            Err(err) => assert!(false, "{}", err),
        }
    }
}
