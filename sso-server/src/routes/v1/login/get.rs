use actix_web::{HttpResponse, web};
use actix_web::http::header::ContentType;
use tera::{Tera, Context};

pub async fn login(tera: web::Data<Tera>) ->HttpResponse{
    let context = Context::new();
    let login_page = tera.render("login.html", &context).unwrap();

    HttpResponse::Ok().content_type(ContentType::html()).body(login_page)
}