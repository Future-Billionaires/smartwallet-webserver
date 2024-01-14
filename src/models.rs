use diesel::prelude::Queryable;
use rocket::serde::Serialize;


#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Credentials {
    pub id: i32,
    pub password_hash: Vec<u8>,
    pub salt: Vec<u8>
}