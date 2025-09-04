use axum::{routing::get, Router, response::Redirect};
use axum::response::{IntoResponse, Response};
use axum::http::{StatusCode, header};
use tower_service::Service;
use worker::*;
use axum::response::Html;
use include_dir::{include_dir, Dir};
use askama_axum::Template;
use std::collections::HashMap;

// Template logic 
mod dps_sims; 
mod mythic_plus;
mod player_metadata;

static ASSETS_DIR: Dir = include_dir!("templates");
//const EVENTS_JSON_URL: &str = "https://r2.seemsgood.org/content/events.json";


fn router() -> Router {
    Router::new() 
        .route("/", get(home_page))
        .route("/about", get(about_page))
        .route("/application", get(apply_page))
        .route("/dps-sims", get(dps_sims::damagesimspage))
        .route("/keys",  get(mythic_plus::mythicplus_page))
        .route("/wowaudit", get(wowaudit_page))
        .route("/css/bulma.min.css", get(bulma_css_handler))
        .route("/events", get(events_handler))
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


// Handler for ../templates/css/bulma.min.css 
async fn bulma_css_handler() -> Response {
    match ASSETS_DIR.get_file("css/bulma.min.css") {
        Some(file) => {
            let body = file.contents_utf8().unwrap_or("").to_string();
            (
                [(header::CONTENT_TYPE, "text/css")],
                body
            ).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            "Bulma CSS file not found".to_string()
        ).into_response()
    }
}


async fn events_handler() -> Response {
    let file = match ASSETS_DIR.get_file("assets/events.json") {
        Some(f) => f,
        None => return (StatusCode::NOT_FOUND, "JSON file not found").into_response(),
    };

    let body = file.contents_utf8().unwrap_or("").to_string();

    (
        [(header::CONTENT_TYPE, "application/json")],
        body
    ).into_response()
}



// Handler for ../templates/assets/events.json
// the file exists on the r2 share but issues with CORS and axum/worker preventing a dynamic
// solution. In the future, we would ideally get new data on page reload.
// currently events.json will only updates on rebuilds.
//
// one possible way to make the data 'appear' new would be pushing events.json to the github
// repo on a timer. this would trigger a automatic build for the cloudflare worker instance.
// makeing the site 'appear' updated.
// async fn events_handler() -> Response {
//     match ASSETS_DIR.get_file("assets/events.json") {
//         Some(file) => {
//             let body = file.contents_utf8().unwrap_or("").to_string();
//             (
//                 [(header::CONTENT_TYPE, "application/json")],
//                 body
//             ).into_response()
//         }
//         None => (
//             StatusCode::NOT_FOUND,
//             "JSON file not found".to_string()
//         ).into_response()
//     }
// }
//
// Home Page
use player_metadata::{build_roster, Player, build_raid, RaidMetaData};
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    show_noti: bool,
    raid_metadata: Vec<RaidMetaData>,
    rosters: HashMap<String, Vec<Player>>, 
}
async fn home_page() -> Html<String> {
    let mut rosters = HashMap::new();
    rosters.insert("Gallywix".to_string(), build_roster("Gallywix"));
    rosters.insert("Kyvesa".to_string(), build_roster("Kyvesa"));
    rosters.insert("Fyrakk".to_string(), build_roster("Fyrakk"));

    let raid_metadata = build_raid();
    let template = IndexTemplate { 
        show_noti: true, 
        raid_metadata,
        rosters,
    };
    let rendered = template.render().unwrap();
    Html(rendered)
}

// Apply Page 
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

// About Page
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

// Spreadsheet Page (wowaudit)
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

