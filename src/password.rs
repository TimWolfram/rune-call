extern crate bcrypt;

use bcrypt::{DEFAULT_COST, verify, hash};
pub const DEFAULT_SALT : &str = "M4CD0N4LD5FR13S";

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    let valid = verify(password, hashed).unwrap();
    valid
}
