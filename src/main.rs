#[macro_use]
extern crate rocket;
extern crate argon2;

use diesel::prelude::*;
use rocket::{Build, Rocket};
use rocket::serde::json::Json;

use self::models::*;
use self::schema::users::dsl::*;

mod database;
mod models;
mod schema;
mod account;
mod hashing;
mod gpt;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![account::login])
        .mount("/", routes![account::create_new_account])
        .mount("/gpt", routes![gpt::send_chat_message])
}