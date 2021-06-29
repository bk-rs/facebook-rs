/// https://developers.facebook.com/docs/graph-api/webhooks/getting-started#verification-requests
use http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Query {
    #[serde(rename = "hub.mode")]
    pub mode: String,
    #[serde(rename = "hub.challenge")]
    pub challenge: i64,
    #[serde(rename = "hub.verify_token")]
    pub verify_token: String,
}

pub fn verify(query_str: &str, verify_token: &str) -> Result<Query, VerifyError> {
    let query: Query = serde_qs::from_str(query_str)?;

    verify_with_query(query, verify_token)
}

pub fn verify_with_query(query: Query, verify_token: &str) -> Result<Query, VerifyError> {
    if query.mode != "subscribe" {
        return Err(VerifyError::ModeMismatch);
    }

    if query.verify_token != verify_token {
        return Err(VerifyError::VerifyTokenMismatch);
    }

    Ok(query)
}

#[derive(thiserror::Error, Debug)]
pub enum VerifyError {
    #[error("QueryInvalid {0}")]
    QueryInvalid(#[from] serde_qs::Error),
    #[error("ModeMismatch")]
    ModeMismatch,
    #[error("VerifyTokenMismatch")]
    VerifyTokenMismatch,
}

pub fn pass_back(query_str: &str, verify_token: &str) -> PassBackResponse {
    match serde_qs::from_str::<Query>(query_str) {
        Ok(query) => pass_back_with_query(query, verify_token),
        Err(err) => PassBackResponse {
            status_code: StatusCode::BAD_REQUEST,
            body: err.to_string(),
        },
    }
}

pub fn pass_back_with_query(query: Query, verify_token: &str) -> PassBackResponse {
    match verify_with_query(query, verify_token) {
        Ok(query) => PassBackResponse {
            status_code: StatusCode::OK,
            body: format!("{}", query.challenge),
        },
        Err(err) => PassBackResponse {
            status_code: StatusCode::BAD_REQUEST,
            body: err.to_string(),
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

    const SAMPLE_QUERY_STR: &str =
        "hub.mode=subscribe&hub.challenge=1158201444&hub.verify_token=meatyhamhock";

    #[test]
    fn test_verify() {
        assert_eq!(
            verify(SAMPLE_QUERY_STR, "meatyhamhock").unwrap().challenge,
            1158201444
        );

        match verify(
            SAMPLE_QUERY_STR
                .replace("hub.mode=", "hub.modexx=")
                .as_str(),
            "meatyhamhock",
        ) {
            Ok(_) => assert!(false),
            Err(VerifyError::QueryInvalid(err)) => {
                assert!(err.to_string().contains("missing field `hub.mode`"))
            }
            Err(err) => assert!(false, "{}", err),
        }

        match verify(
            SAMPLE_QUERY_STR
                .replace("=subscribe&", "=subscribexx&")
                .as_str(),
            "meatyhamhock",
        ) {
            Ok(_) => assert!(false),
            Err(VerifyError::ModeMismatch) => (),
            Err(err) => assert!(false, "{}", err),
        }

        match verify(SAMPLE_QUERY_STR, "FOO") {
            Ok(_) => assert!(false),
            Err(VerifyError::VerifyTokenMismatch) => (),
            Err(err) => assert!(false, "{}", err),
        }
    }

    #[test]
    fn test_pass_back() {
        let res = pass_back(SAMPLE_QUERY_STR, "meatyhamhock");
        assert_eq!(res.status_code, StatusCode::OK);
        assert_eq!(res.body, "1158201444");

        let res = pass_back(SAMPLE_QUERY_STR, "FOO");
        assert_eq!(res.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(res.body, "VerifyTokenMismatch");
    }
}
