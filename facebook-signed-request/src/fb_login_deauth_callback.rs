//! [Official doc](https://developers.facebook.com/docs/facebook-login/manually-build-a-login-flow/#deauth-callback)

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::ParseError;

#[derive(Deserialize, Debug, Clone)]
pub struct Payload {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub user_id: u64,
    pub algorithm: String,
    #[serde(with = "ts_seconds")]
    pub issued_at: DateTime<Utc>,
}
impl crate::Payload for Payload {
    fn algorithm(&self) -> Option<&str> {
        Some(&self.algorithm)
    }
}

pub fn parse(signed_request: &str, app_secret: &str) -> Result<Payload, ParseError> {
    crate::parse(signed_request, app_secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = r#"{"user_id":"0","algorithm":"HMAC-SHA256","issued_at":1624244156}"#;

        match serde_json::from_str::<Payload>(json) {
            Ok(payload) => {
                assert_eq!(payload.user_id, 0);
                assert_eq!(payload.algorithm, "HMAC-SHA256");
                assert_eq!(payload.issued_at.timestamp(), 1624244156);
            }
            Err(err) => assert!(false, "{}", err),
        }
    }
}
