use std::hash::{Hash, Hasher};

use crate::password::hash_password;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub password_hash: String,
    pub jwt: Option<String>,
}

impl Player {
    pub fn new(name: &str, password: &str) -> Player {
        let result = hash_password(password);
        match result {
            Ok(hash) => {
                return Player {
                    id: 0,
                    name: name.to_string(),
                    password_hash: hash,
                    jwt: None
                };
            }
            Err(_) => panic!("Error hashing password"),
        }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
       self.name == other.name
    }
}
impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl Eq for Player {}
