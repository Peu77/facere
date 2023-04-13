use actix_web::{post, get, web, HttpResponse, Responder};
use actix_web::web::Path;
use diesel::RunQueryDsl;
use uuid::Uuid;
use diesel::prelude::*;

use crate::database::Database;
use crate::database::models::user_model::{NewUser, User};
use crate::schema;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user")
        .service(create_user)
        .service(get_user)
    );
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

#[get("/get/{id}")]
async fn get_user(path: Path<Uuid>, data: web::Data<Database>) -> impl Responder {
    let id = path.into_inner();

    let user: Option<User> = schema::users::table
        .filter(schema::users::uuid.eq(id))
        .first::<User>(&mut data.connection.get().unwrap()).optional()
        .unwrap();

    if user.is_none() {
        return HttpResponse::NotFound().body("User not found");
    }

    HttpResponse::Ok().json(user)
}
