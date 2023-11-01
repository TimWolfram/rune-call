use std::{collections::HashMap, sync::atomic::AtomicUsize};
use rocket::{tokio::sync::Mutex, http::CookieJar};
use crate::{model::login::{User, UserId, LoginToken, Role}, controller::password};

type Jwt = String;
type JwtError = &'static str;
type Map<K, V> = Mutex<HashMap<K, V>>;

pub struct UserRepository {
    pub users: Map<UserId, User>,
    pub usernames: Map<String, UserId>,
    user_count: AtomicUsize,
}
impl Default for UserRepository {
    fn default() -> Self {
        let default_admin = User{
                        id: 0, 
                        username: "admin".to_string(),
                        password_hash: password::hash_password("@dm1n15tr4t0r!").unwrap(),
                        nickname: "👍Admin👍".to_string(),
                        role: Role::Admin
                    };
        UserRepository {
            users: Mutex::new (
                HashMap::from (
                    [(0, default_admin)]
                )
            ),
            usernames: Mutex::new(HashMap::new()),
            user_count: AtomicUsize::new(1),
        }
    }
}
impl UserRepository {
    pub async fn create_user<'a>(&'a self, username: &str, password: &'a str, role: Role) -> Result<User, &'a str> {
        let username = username.to_string();
        let usernames = &mut self.usernames.lock().await;
        if usernames.contains_key(&username) {
            return Err("Username already taken!");
        }
        let password_hash = password::hash_password(password)?;
        let id = self.user_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let user = User{id, username: username.to_string(), password_hash, nickname: username.to_string(), role};
        self.users.lock().await.insert(id, user.clone());
        usernames.insert(username, id);
        Ok(user)
    }
    pub async fn remove_user(&self, id: UserId) -> Option<User> {
        self.users.lock().await.remove(&id)
    }
    pub async fn get(&self, id: UserId) -> Option<User> {
        self.users.lock().await.get(&id).cloned()
    }
    pub async fn get_by_username(&self, username: &str) -> Option<User> {
        let usernames = &self.usernames.lock().await;
        let user_id = usernames.get(username);
        match user_id {
            Some(user_id) => return self.users.lock().await.get(&user_id).cloned(),
            None => None
        }
    }
}