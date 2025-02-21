use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;
use axum::response::Html;
use include_dir::{include_dir, Dir};
use askama_axum::Template;

// Template logic for Damage Sims Page (dps_sims.rs)
mod dps_sims;


static _TEMPLATES_DIR: Dir = include_dir!("templates");

fn router() -> Router {
    Router::new() 
        .route("/", get(homepage))
        .route("/about", get(aboutpage))
        .route("/application", get(applypage))
        .route("/dps-sims", get(dps_sims::damagesimspage))
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
struct IndexTemplate {
    show_noti: bool,
}


async fn homepage() -> Html<String> {
    let template = IndexTemplate { show_noti: true };
    let rendered = template.render().unwrap();
    
    Html(rendered)
}

#[derive(Template)]
#[template(path = "apply.html")]
struct ApplyTemplate {
    show_noti: bool,
}

async fn applypage() -> Html<String> {
    let template = ApplyTemplate { show_noti: false };
    let rendered = template.render().unwrap();
    Html(rendered)
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    show_noti: bool,
}

async fn aboutpage() -> Html<String> {
    let template = AboutTemplate { show_noti: true };
    let rendered = template.render().unwrap();
    Html(rendered)
}
