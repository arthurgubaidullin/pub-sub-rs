#![allow(clippy::future_not_send)]

use crate::orders;
use axum::{routing::get, Router};
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest, Result};

fn router() -> Router {
    Router::new().route("/", get(root)).merge(orders::router())
}

async fn root() -> &'static str {
    "Hello Worker!"
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}
