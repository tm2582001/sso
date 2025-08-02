use std::collections::HashSet;
use std::sync::LazyLock;

use actix_session::Session;
use actix_web::http::header::{ContentType, LOCATION};
use actix_web::{HttpResponse, web};
use tera::{Context, Tera};

use crate::utils::LoginRequest;

static ALLOWED_DOMAINS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["http://localhost:8080"]));


pub async fn login_form(
    tera: web::Data<Tera>,
    login_request: web::Query<LoginRequest>,
    session: Session,
) -> HttpResponse {
    let context = Context::new();
    let login_page = tera.render("login.html", &context).unwrap();

    let LoginRequest { service_url } = login_request.into_inner();

    if let Some(url) = service_url.as_ref() {
        let requested_url = url::Url::parse(&url).unwrap();
        let scheme = requested_url.scheme();
        let host = requested_url.host_str().unwrap_or("");
        let port = requested_url.port();

        let origin = if let Some(port) = port {
            format!("{}://{}:{}", scheme, host, port)
        } else {
            format!("{}://{}", scheme, host)
        };

        if !ALLOWED_DOMAINS.contains(origin.as_str()) {
            return HttpResponse::BadRequest().finish();
        }
    }

    if session
        .get::<String>("user")
        .expect("error while getting session")
        .is_some()
        && service_url.is_none()
    {
        return HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .finish();
    }

    if session
        .get::<String>("user")
        .expect("error while getting session")
        .is_some()
    {
        if let Some(url) = service_url {
            // let requested_url = url::Url::parse(&url).unwrap();
            let intrimid = uuid::Uuid::new_v4();

            let redirect_url = format!("{}?ssoToken={}", url, intrimid);

            return HttpResponse::SeeOther()
            .insert_header((LOCATION, redirect_url))
            .finish();
        }
    }

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(login_page)
}
