use actix_web::web;

mod user_service;

pub fn configure(cfg: &mut web::ServiceConfig) {
    vec![
        user_service::configure
    ].iter().for_each(|service| {
        cfg.configure(service);
    });
}