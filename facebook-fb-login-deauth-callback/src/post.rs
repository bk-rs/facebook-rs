use core::{future::Future, pin::Pin};
use std::sync::Arc;

use facebook_signed_request::{
    fb_login_deauth_callback::{parse as signed_request_parse, Payload as SignedRequestPayload},
    ParseError as SignedRequestParseError,
};
use http::StatusCode;

pub const SIGNED_REQUEST_FORM_KEY: &str = "signed_request";

pub type PassBackCallbackFn<'a, C> = Box<
    dyn Fn(
            SignedRequestPayload,
            C,
        ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send>>
        + Send
        + Sync
        + 'a,
>;

pub async fn pass_back<C>(
    request_body_bytes: &[u8],
    app_secret: &str,
    ctx: C,
    callback: Arc<PassBackCallbackFn<'_, C>>,
) -> PassBackResponse {
    match form_urlencoded::parse(request_body_bytes).find(|(k, _)| k == SIGNED_REQUEST_FORM_KEY) {
        Some((_, signed_request)) => {
            pass_back_with_signed_request(signed_request.as_ref(), app_secret, ctx, callback).await
        }
        None => PassBackResponse {
            status_code: StatusCode::BAD_REQUEST,
            body: "".to_owned(),
        },
    }
}

pub async fn pass_back_with_signed_request<C>(
    signed_request: &str,
    app_secret: &str,
    ctx: C,
    callback: Arc<PassBackCallbackFn<'_, C>>,
) -> PassBackResponse {
    match signed_request_parse(signed_request, app_secret) {
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
        Err(err) => match err {
            SignedRequestParseError::EncodedSignatureMissing
            | SignedRequestParseError::PayloadMissing
            | SignedRequestParseError::SignedRequestInvalid
            | SignedRequestParseError::EncodedSignatureBase64DecodeFailed(_)
            | SignedRequestParseError::PayloadBase64DecodeFailed(_) => PassBackResponse {
                status_code: StatusCode::BAD_REQUEST,
                body: err.to_string(),
            },
            _ => PassBackResponse {
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
