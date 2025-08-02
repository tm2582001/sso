use actix_web::http::header::{AUTHORIZATION, ContentType};
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

use crate::models::{Shared,TokenCache};
use crate::utils::JwtData;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    #[serde(rename = "ssoToken")]
    pub sso_token: Option<String>,
}

pub async fn verify_sso_token(
    req: HttpRequest,
    login_request: web::Query<LoginRequest>,
    sso_cache: web::Data<Shared<TokenCache>>
) -> HttpResponse {
    let headers = req.headers();

    match headers.get(AUTHORIZATION) {
        Some(token) => {
            let token_string = token.to_str().unwrap();
            let mut token_iter = token_string.splitn(2, " ");

            match (token_iter.next(), token_iter.next()) {
                (Some("Bearer"), Some(token)) => {
                    if token != "1234" {
                        return HttpResponse::Unauthorized().finish();
                    }
                }
                _ => return HttpResponse::Unauthorized().finish(),
            }
        }
        None => return HttpResponse::Unauthorized().finish(),
    }

    let LoginRequest { sso_token } = login_request.into_inner();

    // if sso_token.is_none() &&

    let ( session_id,app_name) =match &sso_token {
        Some(token) => {
            let cache = sso_cache.lock().unwrap();
            match cache.get_sso_token_details(token){
                None => return HttpResponse::BadRequest().finish(),
                Some(token_details) => token_details
            }
        },
        None => return HttpResponse::BadRequest().finish(),
    };

    let mut cache = sso_cache.lock().unwrap();

    if !cache.is_sso_allowed(&session_id, app_name){
        return HttpResponse::Unauthorized().finish();
    }

    let username = cache.get_username(&session_id);
    
    let payload = JwtData::build(session_id, username);

    let jwt_token = payload.generate_jwt();

    cache.remove_intrim_token(&sso_token.unwrap());

    HttpResponse::Ok().content_type(ContentType::json()).body(format!(r#"{{"token": "{}"}}"#, jwt_token))
}
