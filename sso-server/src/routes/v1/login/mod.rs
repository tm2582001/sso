use actix_web::web;

mod get;

use get::login;

pub fn login_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(login));
}