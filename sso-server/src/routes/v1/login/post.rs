use std::{collections::HashMap, sync::LazyLock};

use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::models::{Shared, TokenCache};
use crate::utils::LoginRequest;

static USERS: LazyLock<HashMap<String, HashMap<String, String>>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    let mut password = HashMap::new();
    password.insert("password".to_string(), "tushar".to_string());
    m.insert("tushar".to_string(), password);
    m
});

#[derive(serde::Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn login(
    login_form: web::Form<LoginForm>,
    login_request: web::Query<LoginRequest>,
    session: Session,
    sso_cache: web::Data<Shared<TokenCache>>,
) -> HttpResponse {
    let LoginForm { username, password } = login_form.into_inner();

    if !USERS
        .get(&username)
        .and_then(|m| m.get("password"))
        .is_some_and(|pass_key| *pass_key == password)
    {
        return HttpResponse::Unauthorized().finish();
    }

    let LoginRequest { service_url } = login_request.into_inner();

    match service_url {
        Some(url) => {
            session.renew();
            let session_id = Uuid::new_v4();
            session.insert("user", session_id).unwrap();
            let mut cache = sso_cache.lock().unwrap();

            cache.store_user_in_cache(session_id.to_string(), username);

            let intrimid = Uuid::new_v4();
            let requested_url = url::Url::parse(&url).unwrap();

            cache.store_application_in_cache(
                requested_url.origin().ascii_serialization(),
                session_id.to_string(),
                intrimid.to_string(),
            );
            let redirect_url = format!("{}?ssoToken={}", url, intrimid);

            HttpResponse::SeeOther()
                .insert_header((LOCATION, redirect_url))
                .finish()
        }
        None => HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .finish(),
    }
}
