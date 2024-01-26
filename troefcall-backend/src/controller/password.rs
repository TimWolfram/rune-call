use bcrypt::{DEFAULT_COST, verify, hash};
use rocket::http::Status;
type Error<'a> = (Status, &'a str);
pub fn hash_password<'a>(password: &str) -> Result<String, Error<'a>> {
    hash(password, DEFAULT_COST).or(Err((Status::InternalServerError, "Error while hashing password!")))
}

pub fn verify_password<'a>(password: &'a str, hashed: &'a str) -> Result<(), Error<'static>> {
    println!("Verifying password: {} with hash: {}", password, hashed);
    let valid = verify(password, hashed).or(Err((Status::InternalServerError, "Error while verifying password!")))?;
    if !valid {
        return Err((Status::Unauthorized, "Incorrect password!"));
    }
    Ok(())
}