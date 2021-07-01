pub use facebook_webhook;

use std::{convert::Infallible, error, sync::Arc};

use bytes::Bytes;
use facebook_webhook::{
    event_notifications::{self, PassBackCallbackFn, SIGNATURE_HEADER_NAME},
    verification_requests::{self, Query},
};
use warp::{
    http::{Response, StatusCode},
    hyper::Body,
    Filter,
};

pub trait Context: Send + Sync + Clone {
    fn get_verify_token(&self, app_id: u64) -> Result<String, Box<dyn error::Error + Send + Sync>>;
    fn get_app_secret(&self, app_id: u64) -> Result<String, Box<dyn error::Error + Send + Sync>>;
}

pub fn handle<C: Context>(
    path_prefix: String,
    ctx: C,
    callback: PassBackCallbackFn<'static, C>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    verification_requests_filter(path_prefix.clone(), ctx.clone()).or(event_notifications_filter(
        path_prefix,
        ctx,
        Arc::new(callback),
    ))
}

fn verification_requests_filter<C: Context>(
    path_prefix: String,
    ctx: C,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(path_prefix)
        .and(warp::path::param::<u64>())
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query::<Query>())
        .and_then(move |app_id, query| {
            let ctx = ctx.clone();

            async move {
                let part: Result<Result<Response<Body>, warp::http::Error>, Infallible> = {
                    match ctx.get_verify_token(app_id) {
                        Ok(verify_token) => {
                            let res =
                                verification_requests::pass_back_with_query(query, &verify_token);

                            Ok(Response::builder()
                                .status(res.status_code)
                                .body(res.body.into()))
                        }
                        Err(err) => Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(err.to_string().into())),
                    }
                };
                part
            }
        })
}

fn event_notifications_filter<C: Context>(
    path_prefix: String,
    ctx: C,
    callback: Arc<PassBackCallbackFn<'static, C>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(path_prefix)
        .and(warp::path::param::<u64>())
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::header::<String>(SIGNATURE_HEADER_NAME))
        .and(warp::body::content_length_limit(1024 * 32).and(warp::body::bytes()))
        .and_then(
            move |app_id, signature_header_value: String, request_body_bytes: Bytes| {
                let ctx = ctx.clone();
                let callback = callback.clone();

                async move {
                    let part: Result<Result<Response<Body>, warp::http::Error>, Infallible> = {
                        match ctx.get_app_secret(app_id) {
                            Ok(app_secret) => {
                                let res = event_notifications::pass_back(
                                    signature_header_value.as_bytes(),
                                    &request_body_bytes[..],
                                    &app_secret,
                                    ctx,
                                    callback,
                                )
                                .await;

                                Ok(Response::builder().status(res.status_code).body("".into()))
                            }
                            Err(err) => Ok(Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(err.to_string().into())),
                        }
                    };
                    part
                }
            },
        )
}
