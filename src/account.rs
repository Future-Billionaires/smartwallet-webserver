use diesel::{ExpressionMethods, insert_into, Queryable, QueryDsl, QueryResult, RunQueryDsl};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::database;
use crate::models::{Credentials, User};
use rocket::response::status::{Accepted, Custom};
use rocket::serde::Serialize;
use serde::Deserialize;
use crate::schema::credentials::dsl::credentials;
use crate::schema::users::dsl::users;
use crate::hashing;

#[derive(Serialize, Queryable, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginData {
    username: String,
    password: String
}

fn get_user(search_username: String) -> QueryResult<User> {
    let connection = &mut database::establish_connection(); // Connect to the database

    users
        .filter(crate::schema::users::username.eq(&search_username))
        .first::<User>(connection)
}

#[post("/create_new_account", data = "<new_user_data>", format="application/json")]
pub fn create_new_account(new_user_data: Json<LoginData>) -> Result<Accepted<String>, Custom<String>> {
    use crate::schema::users::dsl::*;
    use crate::schema::credentials::dsl::*;

    println!("Attempting to create a new user account");

    let connection = &mut database::establish_connection(); // Connect to the database

    println!("Connected to planetscale database");

    insert_into(users).values(username.eq(&new_user_data.username)).execute(connection).expect("Failed to create new user in table users"); // TODO: Handle exception
    let user: QueryResult<User> = get_user(new_user_data.username.clone()); // Get the user's ID for credential matching, I think that taking the ID mysql auto generates will ensure that this works, but it requires an extra db call so TODO: optimize

    // get the id from the User, and check that it exists (it should)
    let user_id: i32 = match user {
        Ok(db_user) => db_user.id,
        Err(_) => return Err(Custom(Status::NotFound, "Could not find newly created user in db, try creating a new account again".to_owned()))
    };

    println!("Added user to users table");
    // Generate salt for hashing, 16 bytes
    let user_salt = hashing::generate_salt();

    println!("Generated salt: {}", user_salt);

    let hashed_user_password = hashing::hash_password(new_user_data.password.clone(), user_salt.clone());

    println!("Hashed password: {}", hashed_user_password);

    insert_into(credentials).values(
        (
            crate::schema::credentials::columns::id.eq(user_id),
            password_hash.eq(hashed_user_password.as_bytes()),
            salt.eq(user_salt.to_string().as_bytes()))
    ).execute(connection).expect("Failed to create new user credentials in credentials table"); // TODO: Handle exception

    println!("Added credentials to the credentials table");

    Ok(Accepted("Successfully created new user account".to_owned()))
}

// Handle login requests
#[post("/login", data = "<login_data>", format="application/json")]
pub fn login(login_data: Json<LoginData>) -> Result<Json<User>, Custom<String>>  {
    println!("Attempting to login to user: {}, with password: {}", login_data.username, login_data.password);
    let connection = &mut database::establish_connection(); // Connect to the database

    println!("Connected to planet scale db");

    // Find the user with that username
    let user: QueryResult<User> = get_user(login_data.username.clone());

    // Check that the user exists
    let user = match user {
        Ok(db_user) => db_user,
        Err(_) => return Err(Custom(Status::NotFound, "No User with that username".to_owned()))
    };

    println!("Found user with id: {}", user.id);
 
    let user_id: i32 = user.id;

    // Find the credentials that match that user id
    let user_credentials: Credentials = credentials
        .filter(crate::schema::credentials::id.eq(user_id))
        .first::<Credentials>(connection)
        .expect("Failed to find credentials attached to the user (probably will never happen)");

    // TODO: Optimize to a single query rather than two using a join

    let is_correct_password = hashing::verify_password(login_data.password.clone(), String::from_utf8(user_credentials.password_hash).unwrap(), String::from_utf8(user_credentials.salt).unwrap()); //TODO: Figure out why password_hash and salt are swapped

    println!("The user password was: {}", is_correct_password);

    println!("{:?}", Json(&user));

    match is_correct_password {
        true => Ok(Json(user)),
        false => Err(Custom(Status::Unauthorized, "Invalid credentials".to_owned()))
    }
}