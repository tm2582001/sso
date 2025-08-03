use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::http::header::ContentType;
use actix_web::{App, HttpResponse, HttpServer, web};
use env_logger::Env;
use tera::{Context, Tera};
use prometheus::{Encoder, TextEncoder, Registry};
use opentelemetry::global;
use opentelemetry_sdk::metrics::SdkMeterProvider;

use std::env;

use sso_server::routes::v1_routes;
use sso_server::models::TokenCache;
use sso_server::middlewares::MetricsMiddleware;


async fn hello_world(tera: web::Data<Tera>) -> HttpResponse {
    let context = Context::new();
    let html = tera.render("index.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

fn init_metrics() -> Registry {
    let registry = Registry::new();
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(registry.clone())
        .build()
        .unwrap();
    
    let provider = SdkMeterProvider::builder()
        .with_reader(exporter)
        .build();
    
    global::set_meter_provider(provider);
    registry
}

// async fn metrics() -> HttpResponse {
//     let encoder = TextEncoder::new();
//     let metric_families = prometheus::gather();
//     let mut buffer = Vec::new();
//     encoder.encode(&metric_families, &mut buffer).unwrap();

//     HttpResponse::Ok()
//         .content_type("text/plain; version=0.0.4; charset=utf-8")
//         .body(buffer)
// }

async fn metrics(registry: web::Data<Registry>) -> HttpResponse {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4; charset=utf-8")
        .body(buffer)
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let env = env::var("RUST_ENV").unwrap_or_else(|_| "development".into());

    if env == "development" {
        dotenvy::dotenv().ok(); // Only load .env file if in dev
        println!("Loaded .env for development");
    }

    let tera = match Tera::new("src/templates/**/*.html") {
        Ok(t) => web::Data::new(t),
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let secret_key = Key::generate();

    let sso_cache = TokenCache::build_shared();
    let sso_cache = web::Data::new(sso_cache);

    let registry = web::Data::new(init_metrics());
    let meter = global::meter("actix_app");

    HttpServer::new(move || {
        App::new()
            .app_data(tera.clone())
            .app_data(sso_cache.clone())
            .app_data(registry.clone())
            .wrap(MetricsMiddleware::new(meter.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .route("/", web::get().to(hello_world))
            .route("/metrics", web::get().to(metrics))
            .service(web::scope("/v1").configure(v1_routes))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
