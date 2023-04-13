use actix_web::{post, web, HttpResponse, Responder};
use diesel::RunQueryDsl;
use uuid::Uuid;

use crate::database::Database;
use crate::database::models::user_model::{NewUser, User};
use crate::schema;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user")
        .service(create_user));
}

#[post("/new")]
async fn create_user(data: web::Data<Database>) -> impl Responder {
    let new_user = NewUser {
        uuid: &Uuid::new_v4(),
        name: "John Doe",
        email: "test",
        password: "test",
    };


    let user: User = diesel::insert_into(schema::users::table)
        .values(&new_user)
        .get_result(&mut data.connection.get().unwrap())
        .expect("Error saving new user");

    println!("User: {:?}", user);


    HttpResponse::Ok().body("created user")
}