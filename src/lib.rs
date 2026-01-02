use axum::{routing::get, Router, response::Redirect};
use axum::response::IntoResponse;
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
mod about_data;

// Include html, css, and and media in local repo.
static ASSETS_DIR: Dir = include_dir!("templates");

// R2 Endpoints for dynamic content
const EVENTS_JSON_URL: &str = "https://r2.seemsgood.org/content/events.json";
const PROGRESS_JSON_URL: &str = "https://r2.seemsgood.org/content/progress.json";
const RAIDER_EXPECTATIONS_URL: &str = "https://docs.google.com/document/export?format=html&id=12LaB7RW0bicUY7Emqz5s6Q-jJlPvGWLjrD8uTCeGTLQ";


// All routes for webpage that are not dynamic.
fn router() -> Router {
    Router::new() 
        .route("/", get(home_page))
        .route("/about", get(about_page))
        .route("/application", get(apply_page))
        .route("/dps-sims", get(dps_sims::damagesimspage))
        .route("/keys",  get(mythic_plus::mythicplus_page))
        .route("/wowaudit", get(wowaudit_page))
        .route("/talents", get(talents_page))
        .route("/resources", get(resources_page))
        .route("/css/bulma.min.css", get(bulma_css_handler))
        .fallback(Redirect::permanent("/"))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    
    // Handle /events and /progress routes manually before passing to router
    let path = req.uri().path();
   
    if path == "/progress" {
        return Ok(fetch_json_endpoint(PROGRESS_JSON_URL, "assets/progress.json").await);
    }
    if path == "/events" {
        return Ok(fetch_json_endpoint(EVENTS_JSON_URL, "assets/events.json").await);
    }
    // Handle /expectations
    if path == "/expectations" {
        return Ok(fetch_html_endpoint(RAIDER_EXPECTATIONS_URL, "assets/404.html").await);
    }

    // For all other routes, use the router
    Ok(router().call(req).await?)
}

async fn fetch_html_endpoint(url: &str, fallback_path: &str) -> axum::http::Response<axum::body::Body> {

    let html_data = match fetch_html_from_whitelist(url).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error fetching HTML from {}: {}", url, e);
            match ASSETS_DIR.get_file(fallback_path) {
                Some(file) => file.contents_utf8().unwrap_or("").to_string(),
                None => "<p>Content unavailable.</p>".to_string(),
            }
        }
    };

    (
        [
        (header::CONTENT_TYPE, "text/html; charset=utf-8"),
        (header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"),
        ],
        html_data,
    )
        .into_response()
}

async fn fetch_html_from_whitelist(url: &str) -> Result<String, String> {
    if url != RAIDER_EXPECTATIONS_URL {
        console_log!(
            "SECURITY: Blocked attempt to fetch non-whitelisted HTML URL: {}",
            url
        );
        return Err("URL not whitelisted".into());
    }

    let mut request_init = RequestInit::new();
    request_init.with_method(Method::Get);

    let request = Request::new_with_init(url, &request_init)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;

    let mut response = Fetch::Request(request)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch HTML: {:?}", e))?;

    let status = response.status_code();
    if status < 200 || status >= 300 {
        return Err(format!("HTML request failed with status: {}", status));
    }

    response
        .text()
        .await
        .map_err(|e| format!("Failed to read HTML response: {:?}", e))
}


// Handler for ../templates/css/bulma.min.css 
async fn bulma_css_handler() -> axum::http::Response<axum::body::Body> {
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

// Generic to fetch JSON from endpoint with fallback
async fn fetch_json_endpoint(url: &str, fallback_path: &str) -> axum::http::Response<axum::body::Body> {
    // Try to fetch from API
    let json_data = match fetch_from_r2(url).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error fetching from {}: {}", url, e);
            // Fallback to static file
            match ASSETS_DIR.get_file(fallback_path) {
                Some(file) => file.contents_utf8().unwrap_or("{}").to_string(),
                None => "{}".to_string(),
            }
        }
    };
    
    (
        [
            (header::CONTENT_TYPE, "application/json"),
            (header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"),
        ],
        json_data
    ).into_response()
}

// Helper to fetch JSON from R2
async fn fetch_from_r2(url: &str) -> std::result::Result<String, String> {
    // Verify input of url here. 
    if url != EVENTS_JSON_URL && url != PROGRESS_JSON_URL {
        console_log!("SECURITY: Blocked attempt to fetch from non-whitelisted URL: {}", url);
        return Err(format!("URL not whitelisted: {}", url));
    }

    // Helpful debugging runtime requests.
    // console_log!("=== FETCH REQUEST DEBUG ===");
    // console_log!("Target URL: {}", url);
    // console_log!("URL matches EVENTS? {}", url == EVENTS_JSON_URL);
    // console_log!("URL matches PROGRESS? {}", url == PROGRESS_JSON_URL);
    // console_log!("========================");
    
    let mut request_init = RequestInit::new();
    request_init.with_method(Method::Get);
    
    let request = Request::new_with_init(url, &request_init)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;
    
    let mut response = Fetch::Request(request)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch: {:?}", e))?;
    
    let status = response.status_code();
    if status < 200 || status >= 300 {
        return Err(format!("Request failed with status: {}", status));
    }
    
    response
        .text()
        .await
        .map_err(|e| format!("Failed to read response text: {:?}", e))
}


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
    rosters.insert("Dimensius".to_string(), build_roster("Dimensius"));
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
use about_data::{ContactInfo, build_contacts};
#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    show_noti: bool,
    contacts: Vec<ContactInfo>,
}
async fn about_page() -> Html<String> {
    let contacts = build_contacts();
    let template = AboutTemplate { 
        show_noti: true,
        contacts,
    };
    let rendered = template.render().unwrap();
    Html(rendered)
}

// Spreadsheet Page (wowaudit)
// TODO link to /resources
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

// Talents Page (talents.seemsgood.org)
#[derive(Template)]
#[template(path = "talents.html")]
struct TalentsTemplate {
    show_noti: bool,
}
async fn talents_page() -> Html<String> {
    let template = TalentsTemplate { show_noti: true };
    let rendered = template.render().unwrap();
    Html(rendered)
}

// Resources Page (Raider expectations, loot management, trial process, and raid schedule.)
#[derive(Template)]
#[template(path = "resources.html")]
struct ResourcesTemplate{
    show_noti: bool,
}
async fn resources_page() -> Html<String> {
    let template = ResourcesTemplate { show_noti: true };
    let rendered = template.render().unwrap();
    Html(rendered)
}

