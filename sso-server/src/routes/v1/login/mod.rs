use actix_web::web;

mod get;
mod post;

use get::login_form;
use post::login;

pub fn login_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(login_form));
    cfg.route("", web::post().to(login));
}