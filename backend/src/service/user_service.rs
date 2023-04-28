use actix_web::{post, get, web, HttpResponse, Responder, Scope};
use actix_web::guard::{Guard, GuardContext};
use actix_web::web::Path;

use uuid::Uuid;
use diesel::prelude::*;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

#[post("/register")]
async fn create_user(data: web::Data<AppState>, user_create_dto: web::Json<UserCreateDto>) -> impl Responder {
    // check if user already exists
    if get_user_by_email(user_create_dto.email.as_str(), &data.db).await.is_some() {
        return HttpResponse::BadRequest().json({
            "User already exists"
        });
    }

    let hashed_password = bcrypt::hash(user_create_dto.password.as_str()).unwrap();
    let new_user = NewUser {
        uuid: Uuid::new_v4(),
        name: &user_create_dto.name,
        email: &user_create_dto.email,
        password: hashed_password,
    };

    // insert new user
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(&mut data.db.connection.get().unwrap())
        .expect("Error saving new user");

    let token = generate_token(&user_create_dto.email, &data.secret);

    HttpResponse::Ok().json(TokenResponse {
        token
    })
}

#[derive(Deserialize)]
struct UserLoginDto {
    email: String,
    password: String,
}

#[post("/login")]
async fn verify_user(data: web::Data<AppState>, user_verify_dto: web::Json<UserLoginDto>) -> impl Responder {
    // fetch user to verify the password
    let user_option: Option<User> = get_user_by_email(user_verify_dto.email.as_str(), &data.db).await;

    let user = match user_option {
        Some(user) => user,
        None => return HttpResponse::NotFound().body("User not found"),
    };

    let is_password_valid: bool = bcrypt::verify(user_verify_dto.password.as_str(), user.password.as_str());

    match is_password_valid {
        true => HttpResponse::Ok().json(TokenResponse {
            token: generate_token(&user.email, &data.secret)
        }),
        false => HttpResponse::Unauthorized().body("User not verified")
    }
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
        .filter(users::columns::email.eq(email))
        .first(&mut db.connection.get().unwrap())
        .optional()
        .expect("Error loading user")
}

pub async fn get_user_by_uuid(uuid: Uuid, db: &Database) -> Option<User> {
    schema::users::table
        .filter(users::columns::uuid.eq(uuid))
        .first(&mut db.connection.get().unwrap())
        .optional()
        .expect("Error loading user")
}