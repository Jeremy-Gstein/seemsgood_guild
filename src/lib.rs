use axum::{routing::get, Router, response::Redirect};
use axum::response::{IntoResponse, Response};
use axum::http::{StatusCode, header};
use tower_service::Service;
use worker::*;
use axum::response::Html;
use include_dir::{include_dir, Dir};
use askama_axum::Template;

// Template logic for Damage Sims Page (dps_sims.rs)
mod dps_sims;
mod mythic_plus;


//static _TEMPLATES_DIR: Dir = include_dir!("templates");
static ASSETS_DIR: Dir = include_dir!("templates");

fn router() -> Router {
    Router::new() 
        .route("/", get(home_page))
        .route("/about", get(about_page))
        .route("/application", get(apply_page))
        .route("/dps-sims", get(dps_sims::damagesimspage))
        .route("/keys",  get(mythic_plus::mythicplus_page))
        .route("/wowaudit", get(wowaudit_page))
        .route("/css/carousel.css", get(css_handler))
        .fallback(Redirect::permanent("/"))
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


// Handler for ../templates/css/carousel.css
async fn css_handler() -> Response {
    match ASSETS_DIR.get_file("css/carousel.css") {
        Some(file) => {
            let body = file.contents_utf8().unwrap_or("").to_string();
            (
                [(header::CONTENT_TYPE, "text/css")],
                body
            ).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            "CSS file not found".to_string()
        ).into_response()
    }
}


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    show_noti: bool,
}

async fn home_page() -> Html<String> {
    let template = IndexTemplate { show_noti: true };
    let rendered = template.render().unwrap();
    Html(rendered)
}

#[derive(Template)]
#[template(path = "apply.html")]
struct ApplyTemplate {
    show_noti: bool,
}

async fn apply_page() -> Html<String> {
    let template = ApplyTemplate { show_noti: false };
    let rendered = template.render().unwrap();
    Html(rendered)
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    show_noti: bool,
}

async fn about_page() -> Html<String> {
    let template = AboutTemplate { show_noti: true };
    let rendered = template.render().unwrap();
    Html(rendered)
}

#[derive(Template)]
#[template(path = "wowaudit.html")]
struct WowauditTemplate {
    show_noti: bool,
}

async fn wowaudit_page() -> Html<String> {
    let template = WowauditTemplate { show_noti: true };
    let rendered = template.render().unwrap();
    Html(rendered)
}

