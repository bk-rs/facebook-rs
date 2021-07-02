pub use facebook_fb_login_deauth_callback;

use std::{collections::HashMap, convert::Infallible, error, sync::Arc};

use facebook_fb_login_deauth_callback::{
    get::PASS_BACK_STATUS_CODE,
    post::{pass_back_with_signed_request, PassBackCallbackFn, SIGNED_REQUEST_FORM_KEY},
};
use warp::{
    http::{Response, StatusCode},
    hyper::Body,
    Filter,
};

pub trait Context: Send + Sync + Clone {
    fn get_app_secret(&self, app_id: u64) -> Result<String, Box<dyn error::Error + Send + Sync>>;
}

pub fn handle<C: Context>(
    path_prefix: String,
    ctx: C,
    callback: PassBackCallbackFn<'static, C>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_filter(path_prefix.clone(), ctx.clone()).or(post_filter(
        path_prefix,
        ctx,
        Arc::new(callback),
    ))
}

fn get_filter<C: Context>(
    path_prefix: String,
    ctx: C,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(path_prefix)
        .and(warp::path::param::<u64>())
        .and(warp::path::end())
        .and(warp::get())
        .and_then(move |app_id| {
            let ctx = ctx.clone();

            async move {
                let part: Result<Result<Response<Body>, warp::http::Error>, Infallible> = {
                    match ctx.get_app_secret(app_id) {
                        Ok(_) => Ok(Response::builder()
                            .status(PASS_BACK_STATUS_CODE)
                            .body("".into())),
                        Err(err) => Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(err.to_string().into())),
                    }
                };
                part
            }
        })
}

fn post_filter<C: Context>(
    path_prefix: String,
    ctx: C,
    callback: Arc<PassBackCallbackFn<'static, C>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(path_prefix)
        .and(warp::path::param::<u64>())
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 32).and(warp::body::form()))
        .and_then(move |app_id: u64, request_form: HashMap<String, String>| {
            let ctx = ctx.clone();
            let callback = callback.clone();

            async move {
                let part: Result<Result<Response<Body>, warp::http::Error>, Infallible> = {
                    match request_form.get(SIGNED_REQUEST_FORM_KEY) {
                        Some(signed_request) => match ctx.get_app_secret(app_id) {
                            Ok(app_secret) => {
                                let res = pass_back_with_signed_request(
                                    signed_request,
                                    &app_secret,
                                    ctx,
                                    callback,
                                )
                                .await;

                                Ok(Response::builder()
                                    .status(res.status_code)
                                    .body(res.body.into()))
                            }
                            Err(err) => Ok(Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(err.to_string().into())),
                        },
                        None => Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body("form invalid".into())),
                    }
                };
                part
            }
        })
}
