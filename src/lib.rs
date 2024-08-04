use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;
use axum::response::Html;
use include_dir::{include_dir, Dir};
use askama_axum::Template;

static _TEMPLATES_DIR: Dir = include_dir!("templates");





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

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn homepage() -> Html<String> {
    // `std::include_str` macro can be used to include an utf-8 file as `&'static str` in compile
    // time. This method is relative to current `main.rs` file.
//    let _ = TEMPLATES_DIR;
    let template = IndexTemplate;
    let rendered = template.render().unwrap();
    
    Html(rendered)
}

#[derive(Template)]
#[template(path = "apply.html")]
struct ApplyTemplate;




async fn applypage() -> Html<String> {
    // `std::include_str` macro can be used to include an utf-8 file as `&'static str` in compile
    // time. This method is relative to current `main.rs` file.
    let template = ApplyTemplate;
    let rendered = template.render().unwrap();
    Html(rendered)
}
