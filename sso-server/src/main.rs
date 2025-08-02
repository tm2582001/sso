use actix_web::http::header::ContentType;
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::cookie::Key;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use env_logger::Env;
use tera::{Context, Tera};

use sso_server::routes::v1_routes;

async fn hello_world(tera: web::Data<Tera>) -> HttpResponse {
    let context = Context::new();
    let html = tera.render("index.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let tera = match Tera::new("src/templates/**/*.html") {
        Ok(t) => web::Data::new(t),
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let secret_key = Key::generate();


    HttpServer::new(move || {
        App::new()
            .app_data(tera.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .route("/", web::get().to(hello_world))
            .service(
                web::scope("/v1")
                    .configure(v1_routes)
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
