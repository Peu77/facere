use diesel::prelude::*;

use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub uuid: &'a Uuid,
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}