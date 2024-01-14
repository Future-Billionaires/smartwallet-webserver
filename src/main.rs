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

// TODO: Remove this before pushing to prod :)
#[get("/")]
fn index() -> Json<Vec<User>> {
    let connection = &mut database::establish_connection();
    users.load::<User>(connection).map(Json).expect("Error loading users")
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![account::login])
        .mount("/", routes![account::create_new_account])
}