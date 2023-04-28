use actix::dev::Request;
use actix_web::{post, get, web, HttpResponse, Responder, Scope};
use actix_web::guard::{Guard, GuardContext};
use actix_web::web::Path;

use uuid::Uuid;
use diesel::prelude::*;
use pwhash::bcrypt;
use serde::Deserialize;

use crate::database::Database;
use crate::database::models::user_model::{NewUser, User};
use crate::schema;
use crate::middleware::{AuthenticationMiddleware, generate_token};
use crate::AppState;
use crate::schema::users;

pub fn user_service() -> Scope {
    web::scope("/user")
        .service(create_user)
        .service(verify_user)
        .service(get_user)
        .service(test)
}


#[get("/test")]
async fn test(auth: AuthenticationMiddleware) -> impl Responder {
    HttpResponse::Ok().body(auth.user.email)
    // update user name
}

#[derive(Deserialize)]
struct UserCreateDto {
    name: String,
    email: String,
    password: String,
}

#[post("/register")]
async fn create_user(data: web::Data<AppState>, user_create_dto: web::Json<UserCreateDto>) -> impl Responder {
    let hashed_password = bcrypt::hash(user_create_dto.password.as_str()).unwrap();
    let new_user = NewUser {
        uuid: Uuid::new_v4(),
        name: user_create_dto.name.clone(),
        email: user_create_dto.email.clone(),
        password: hashed_password,
    };

    let user: User = diesel::insert_into(schema::users::table)
        .values(new_user)
        .get_result(&mut data.db.connection.get().unwrap())
        .expect("Error saving new user");

    println!("User: {:?}", user);

    let token = generate_token(&user.email, &data.secret);

    HttpResponse::Ok().json({
        token
    })
}

#[derive(Deserialize)]
struct UserVerifyDto {
    email: String,
    password: String,
}

#[post("/verify")]
async fn verify_user(data: web::Data<AppState>, user_verify_dto: web::Json<UserVerifyDto>) -> impl Responder {
    let user: Option<User> = get_user_by_email(user_verify_dto.email.as_str(), &data.db).await;

    let user = match user {
        Some(user) => user,
        None => return HttpResponse::NotFound().body("User not found"),
    };

    let is_valid: bool = bcrypt::verify(user_verify_dto.password.as_str(), user.password.as_str());

    return if is_valid {
        HttpResponse::Ok().body("User verified")
    } else {
        HttpResponse::Unauthorized().body("User not verified")
    };
}

#[get("/get/{id}")]
async fn get_user(path: Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();

    let user: Option<User> = get_user_by_uuid(id, &data.db).await;

    return match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    };
}

pub async fn get_user_by_email(email: &str, db: &Database) -> Option<User> {
    schema::users::table
        .filter(schema::users::columns::email.eq(email))
        .first(&mut db.connection.get().unwrap())
        .optional()
        .expect("Error loading user")
}

pub async fn get_user_by_uuid(uuid: Uuid, db: &Database) -> Option<User> {
    schema::users::table
        .filter(schema::users::columns::uuid.eq(uuid))
        .first(&mut db.connection.get().unwrap())
        .optional()
        .expect("Error loading user")
}