use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;
use axum::response::Html;
use include_dir::{include_dir, Dir};
use askama_axum::Template;

mod dps_sims;
pub use dps_sims::*;


static _TEMPLATES_DIR: Dir = include_dir!("templates");

fn router() -> Router {
    Router::new() 
        .route("/", get(homepage))
        .route("/application", get(applypage))
        .route("/damagesims", get(damagesimspage))
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
    let template = IndexTemplate;
    let rendered = template.render().unwrap();
    
    Html(rendered)
}

#[derive(Template)]
#[template(path = "apply.html")]
struct ApplyTemplate;

async fn applypage() -> Html<String> {
    let template = ApplyTemplate;
    let rendered = template.render().unwrap();
    Html(rendered)
}
