use bcrypt::{DEFAULT_COST, verify, hash};

pub fn hash_password(password: &str) -> Result<String, &'static str> {
    hash(password, DEFAULT_COST).or(Err("Error while hashing password!"))
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    let valid = verify(password, hashed).unwrap();
    valid
}