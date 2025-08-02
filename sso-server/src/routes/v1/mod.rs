use actix_web::web;

mod login;
mod verify_sso_token;

pub use login::login_routes;
pub use verify_sso_token::verify_sso_token;

pub fn v1_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/login").configure(login_routes));
    cfg.route("/verify_sso", web::get().to(verify_sso_token));
}
