use actix_web::web;

mod login;

pub use login::login_routes;

pub fn v1_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/login").configure(login_routes));
}
