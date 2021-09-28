use hmac::{Hmac, Mac as _, NewMac as _};
use serde::de::DeserializeOwned;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[cfg(feature = "with-data-deletion-callback")]
pub mod data_deletion_callback;
#[cfg(feature = "with-fb-login-deauth-callback")]
pub mod fb_login_deauth_callback;

pub const NORMALLY_ALGORITHM: &str = "HMAC-SHA256";

pub trait Payload: DeserializeOwned {
    fn algorithm(&self) -> Option<&str> {
        None
    }
}

/// [Official doc](https://developers.facebook.com/docs/games/gamesonfacebook/login#parsingsr)
pub fn parse<T: Payload>(signed_request: &str, app_secret: &str) -> Result<T, ParseError> {
    let mut signed_request_split = signed_request.split('.');
    let encoded_sig = signed_request_split
        .next()
        .ok_or(ParseError::EncodedSignatureMissing)?;
    let payload = signed_request_split
        .next()
        .ok_or(ParseError::PayloadMissing)?;
    if signed_request_split.next().is_some() {
        return Err(ParseError::SignedRequestInvalid);
    }

    let sig = base64::decode_config(encoded_sig, base64::URL_SAFE)
        .map_err(ParseError::EncodedSignatureBase64DecodeFailed)?;
    let data = base64::decode_config(payload, base64::URL_SAFE)
        .map_err(ParseError::EncodedSignatureBase64DecodeFailed)?;

    let data: T = serde_json::from_slice(&data).map_err(ParseError::PayloadJsonDecodeFailed)?;

    let algorithm = data.algorithm().unwrap_or(NORMALLY_ALGORITHM);

    let expected_sig = match algorithm {
        NORMALLY_ALGORITHM => hmac_sha256_payload(payload.as_bytes(), app_secret)
            .map_err(|_| ParseError::SignatureCalculateFailed)?,
        _ => return Err(ParseError::AlgorithmUnknown(algorithm.to_owned())),
    };

    if sig != expected_sig {
        return Err(ParseError::SignatureMismatch);
    }

    Ok(data)
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("EncodedSignatureMissing")]
    EncodedSignatureMissing,
    #[error("PayloadMissing")]
    PayloadMissing,
    #[error("SignedRequestInvalid")]
    SignedRequestInvalid,
    #[error("EncodedSignatureBase64DecodeFailed {0}")]
    EncodedSignatureBase64DecodeFailed(base64::DecodeError),
    #[error("PayloadBase64DecodeFailed {0}")]
    PayloadBase64DecodeFailed(base64::DecodeError),
    #[error("PayloadJsonDecodeFailed {0}")]
    PayloadJsonDecodeFailed(serde_json::Error),
    #[error("AlgorithmUnknown {0}")]
    AlgorithmUnknown(String),
    #[error("SignatureCalculateFailed")]
    SignatureCalculateFailed,
    #[error("SignatureMismatch")]
    SignatureMismatch,
}

// $ echo -n "value" | openssl sha256 -hmac "key"
// (stdin)= 90fbfcf15e74a36b89dbdb2a721d9aecffdfdddc5c83e27f7592594f71932481
fn hmac_sha256_payload(payload_bytes: &[u8], app_secret: &str) -> Result<Vec<u8>, String> {
    let mut hmac =
        HmacSha256::new_from_slice(app_secret.as_bytes()).map_err(|err| err.to_string())?;
    hmac.update(payload_bytes);
    let hmac_result = hmac.finalize().into_bytes();
    let sig = hmac_result.to_vec();

    Ok(sig)
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;

    #[test]
    fn test_parse() {
        #[derive(Deserialize)]
        struct MyPayload {
            user_id: String,
            algorithm: String,
            issued_at: u64,
        }
        impl Payload for MyPayload {
            fn algorithm(&self) -> Option<&str> {
                Some(&self.algorithm)
            }
        }

        // echo -n '{"user_id":"0","algorithm":"HMAC-SHA256","issued_at":1624244156}' | base64 | tr '+/' '-_' | tr -d '='
        // echo -n 'eyJ1c2VyX2lkIjoiMCIsImFsZ29yaXRobSI6IkhNQUMtU0hBMjU2IiwiaXNzdWVkX2F0IjoxNjI0MjQ0MTU2fQ' | openssl sha256 -hmac "key" -binary | base64 | tr '+/' '-_' | tr -d '='
        let signed_request = "Mf_s6nTb38UYqioBmPqu0Ewm9souPZB9I2fIGwV729U.eyJ1c2VyX2lkIjoiMCIsImFsZ29yaXRobSI6IkhNQUMtU0hBMjU2IiwiaXNzdWVkX2F0IjoxNjI0MjQ0MTU2fQ";

        match parse::<MyPayload>(signed_request, "key") {
            Ok(payload) => {
                assert_eq!(payload.user_id, "0");
                assert_eq!(payload.algorithm, "HMAC-SHA256");
                assert_eq!(payload.issued_at, 1624244156);
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn test_hmac_sha256_payload() {
        assert_eq!(
            hex::encode(hmac_sha256_payload(b"value", "key").unwrap()),
            "90fbfcf15e74a36b89dbdb2a721d9aecffdfdddc5c83e27f7592594f71932481"
        );
    }
}
