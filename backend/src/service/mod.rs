use actix_web::web;

pub mod user_service;

pub fn configure(cfg: &mut web::ServiceConfig) {
    [
        user_service::user_service
    ].iter().for_each(|service| {
        cfg.service(service());
    });
}