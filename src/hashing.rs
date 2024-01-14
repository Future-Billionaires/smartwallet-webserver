use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

// Hash password using Argon2 hashing algorithm
pub fn hash_password(password: String, salt: SaltString) -> String {
    Argon2::default().hash_password(password.as_ref(), salt.as_salt()).expect("Failed to hash password").to_string() //TODO: Handle exception
}

//Checks a plain text password against its hash
pub fn verify_password(plain_password: String, hashed_password: String, salt: String) -> bool {
    println!("Attempting to verify: {plain}, against hash: {hash}", plain=plain_password, hash=hashed_password);

    // Hash the plain text password
    let parsed_hash = PasswordHash::new(hashed_password.as_ref()).expect("Failed to verify passwords: Unable to create new password hash from db"); //TODO: Handle exception

    Argon2::default().verify_password(plain_password.as_ref(), &parsed_hash).is_ok()
}
