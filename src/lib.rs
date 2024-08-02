use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;
use axum::response::Html;



fn router() -> Router {
    Router::new() 
        .route("/", get(homepage))
        .route("/application", get(applypage))
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

async fn homepage() -> Html<&'static str> {
    // `std::include_str` macro can be used to include an utf-8 file as `&'static str` in compile
    // time. This method is relative to current `main.rs` file.
    Html(include_str!("../templates/layout.html"))
}

async fn applypage() -> Html<&'static str> {
    // `std::include_str` macro can be used to include an utf-8 file as `&'static str` in compile
    // time. This method is relative to current `main.rs` file.
    Html(include_str!("../templates/apply.html"))
}
