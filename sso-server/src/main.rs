use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::http::header::ContentType;
use tera::{Tera, Context};


async fn hello_world( tera: web::Data<Tera>)->HttpResponse {
    let context = Context::new();
    let html = tera.render("index.html", &context).unwrap();

    HttpResponse::Ok().content_type(ContentType::html()).body(html)
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

    HttpServer::new(move || App::new()
        .app_data(tera.clone())
    .route("/", web::get().to(hello_world)))
        .bind("127.0.0.1:8000")?
        .run()
        .await?;
    println!("Server started at http://localhost:8000");

    Ok(())
}
