use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result}; // webapp framework
use actix_web::middleware::Logger; // logging middleware (requests)
use tera::Tera; // html templating engine
use lazy_static::lazy_static; //ensure templates only initialized once.
use serde::{Serialize, Deserialize}; // json Parsing
use uuid::Uuid;                                    // uuid

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


struct User {
    id: String,
    name: String,
    realm: String,
}


#[derive(Serialize, Deserialize)]
pub struct FormInput {
    name: String,
    realm: String,
}

#[get("/form")]
async fn guild_application() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("form.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[post("/postup")]
async fn postup(params: web::Form<FormInput>) -> Result<HttpResponse> {
    println!("\n[POST-DATA] - Name:{:?} Realm:{:?}\n",params.name, params.realm);
    let name = &params.name;
    let realm = &params.realm;
    // gererate a new UUID for user
    let id = Uuid::new_v4().to_string();
    // extract forminput data and create a new user.
    // currently printing the data to stdout.
    // this will be stored in a db, or sent directly to discord api.
    let user = User {
        id: id.clone(),
        name: name.clone(),
        realm: realm.clone(),
    };
    println!("[Application] - ID:{:?}, Name:{:?}, Realm:{:?}\n", user.id, user.name, user.realm);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Thanks {}! your application has been processed. a guild member will get back to you soon.", user.name)))
}

#[get("/")]
async fn index() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}




#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let server_address: &str = "127.0.0.1";
    let server_port: u16 = 8080;
    println!("Server Running on http://{}:{}", server_address, server_port);
    HttpServer::new(||  
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(guild_application)
            .service(postup)

    )
    .bind((server_address, server_port))?
    .run()
    .await
}
