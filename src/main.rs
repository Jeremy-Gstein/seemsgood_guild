use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result}; // webapp framework
use actix_web::middleware::Logger; // logging middleware (requests)
use env_logger::Env; // logging
use tera::Tera; // html templating engine
use lazy_static::lazy_static; //ensure templates only initialized once.
use serde::{Serialize, Deserialize}; // json parsing
use std::io::Write; // write to stdout

// lazy_static is used to ensure that the templates are only initialized once.
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        // let source = "templates/**/*";
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing Error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
 }



// form scope
struct AppState {
    state: String,
}


#[derive(Serialize, Deserialize)]
pub struct FormInput {
    name: String,
}


async fn guild_application() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("form.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

async fn postup(params: web::Form<FormInput>) -> Result<HttpResponse> {
    std::io::stdout().write_all(format!("name {:?}", params.name).as_bytes()).unwrap();
    Ok(HttpResponse::Ok()
        // .content_type("text/plain")
        //.body(format!("Your name is {}", params.name))
        .body(params.name.clone()))
}

#[get("/")]
async fn index() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}




#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    let _server_logs = env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(||  
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(web::resource("/form").route(web::get().to(guild_application)))
            .service(web::resource("/postup").route(web::post().to(postup)))

    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
