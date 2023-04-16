use actix_web::{post, get, web, HttpResponse, Responder};
use actix_web::web::Path;
use diesel::RunQueryDsl;
use uuid::Uuid;
use diesel::prelude::*;
use pwhash::bcrypt;
use serde::Deserialize;

use crate::database::Database;
use crate::database::models::user_model::{NewUser, User};
use crate::schema;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user")
        .service(create_user)
        .service(verify_user)
        .service(get_user)
    );
}

#[derive(Deserialize)]
struct UserCreateDto {
    name: String,
    email: String,
    password: String,
}

#[post("/new")]
async fn create_user(data: web::Data<Database>, user_create_dto: web::Json<UserCreateDto>) -> impl Responder {
    let hashed_password = bcrypt::hash(user_create_dto.password.as_str()).unwrap();
    let new_user = NewUser {
        uuid: Uuid::new_v4(),
        name: user_create_dto.name.clone(),
        email: user_create_dto.email.clone(),
        password: hashed_password,
    };

    let user: User = diesel::insert_into(schema::users::table)
        .values(new_user)
        .get_result(&mut data.connection.get().unwrap())
        .expect("Error saving new user");

    println!("User: {:?}", user);


    HttpResponse::Ok().body("created user")
}

#[derive(Deserialize)]
struct UserVerifyDto {
    email: String,
    password: String,
}

#[post("/verify")]
async fn verify_user(data: web::Data<Database>, user_verify_dto: web::Json<UserVerifyDto>) -> impl Responder {
    let user: Option<User> = schema::users::table
        .filter(schema::users::email.eq(user_verify_dto.email.as_str()))
        .first::<User>(&mut data.connection.get().unwrap()).optional()
        .unwrap();

    let user = match user {
        Some(user) => user,
        None => return HttpResponse::NotFound().body("User not found"),
    };

    let is_valid: bool = bcrypt::verify(user_verify_dto.password.as_str(), user.password.as_str());

    return if is_valid {
        HttpResponse::Ok().body("User verified")
    } else {
        HttpResponse::Unauthorized().body("User not verified")
    }
}

#[get("/get/{id}")]
async fn get_user(path: Path<Uuid>, data: web::Data<Database>) -> impl Responder {
    let id = path.into_inner();

    let user: Option<User> = schema::users::table
        .filter(schema::users::uuid.eq(id))
        .first::<User>(&mut data.connection.get().unwrap()).optional()
        .unwrap();

    return match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    };
}
