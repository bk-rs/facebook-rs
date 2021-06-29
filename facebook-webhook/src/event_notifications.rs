/// https://developers.facebook.com/docs/graph-api/webhooks/getting-started#event-notifications
use std::str::{self, FromStr};

use hmac::{Hmac, Mac as _, NewMac as _};
use sha1::Sha1;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_signature() {
        assert_eq!(
            "sha1=2ffc1b81550c62e2c2dc7bdeef8bb40680e1ecf4"
                .parse::<Signature>()
                .unwrap(),
            Signature::Sha1("2ffc1b81550c62e2c2dc7bdeef8bb40680e1ecf4".to_owned())
        );
    }

    #[test]
    fn test_sha1_payload() {
        assert_eq!(
            sha1_payload(b"value", "key").unwrap(),
            "57443a4c052350a44638835d64fd66822f813319"
        );
    }
}
