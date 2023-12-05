use std::{collections::HashMap, sync::atomic::AtomicUsize};
use rocket::tokio::sync::Mutex;
use crate::{model::{login::{User, UserId, Role}, game::Room}, controller::password};

type Map<K, V> = Mutex<HashMap<K, V>>;

pub struct UserRepository {
    pub users: Map<UserId, User>,
    pub usernames: Map<String, UserId>,
    user_count: AtomicUsize,
}
impl Default for UserRepository {
    fn default() -> Self {
        //create default admin user: otherwise, we cannot create any other admins
        let admin_name = "admin".to_string();
        UserRepository {
            users: Mutex::new (
                HashMap::from (
                    [(0, User {
                        id: 0, 
                        username: "admin".to_string(),
                        password_hash: password::hash_password("adminpw!").unwrap(),
                        nickname: "👍Admin👍".to_string(),
                        role: Role::Admin,
                        current_room: None,
                    })]
                )
            ),
            usernames: Mutex::new(HashMap::from([(admin_name, 0)])),
            user_count: AtomicUsize::new(1),
        }
    }
}
impl UserRepository {
    pub fn test_repo() -> Self {
        UserRepository {
            users: Mutex::new(HashMap::from([
                (0, User {
                        id: 0, 
                        username: "admin".to_string(),
                        password_hash: password::hash_password("adminpw!").unwrap(),
                        nickname: "👍Admin👍".to_string(),
                        role: Role::Admin,
                        current_room: None,
                    }),
                (1, User {
                        id: 1, 
                        username: "test_user".to_string(),
                        password_hash: password::hash_password("userpw!").unwrap(),
                        nickname: "User".to_string(),
                        role: Role::Player,
                        current_room: None,
                    }),
                (2, User {
                        id: 2, 
                        username: "test_user2".to_string(),
                        password_hash: password::hash_password("user2pw!").unwrap(),
                        nickname: "test_User2".to_string(),
                        role: Role::Player,
                        current_room: None,
                    }),
                (3, User {
                        id: 3, 
                        username: "test_user3".to_string(),
                        password_hash: password::hash_password("user3pw!").unwrap(),
                        nickname: "User3".to_string(),
                        role: Role::Player,
                        current_room: None,
                    }),
                (4, User {
                        id: 4, 
                        username: "test_user4".to_string(),
                        password_hash: password::hash_password("user4pw!").unwrap(),
                        nickname: "👍User4👍".to_string(),
                        role: Role::Player,
                        current_room: None,
                    }),
                (5, User {
                        id: 5, 
                        username: "test_user5".to_string(),
                        password_hash: password::hash_password("user5pw!").unwrap(),
                        nickname: "👍User5👍".to_string(),
                        role: Role::Player,
                        current_room: None,
                    }),
                (6, User {
                        id: 6, 
                        username: "test_user6".to_string(),
                        password_hash: password::hash_password("user6pw!").unwrap(),
                        nickname: "👍User6👍".to_string(),
                        role: Role::Player,
                        current_room: None,
                    }),
            ])),
            usernames: Mutex::new(HashMap::new()),
            user_count: AtomicUsize::new(1),
        }
    }
}
type ErrType<'a> = &'a str;
type EndpointResult<'a, T> = Result<T, ErrType<'a>>;

impl UserRepository {
    pub async fn create_user<'a>(&'a self, username: &str, password: &'a str, role: Role) -> Result<User, ErrType<'a>> {
        let username = username.to_string();
        let usernames = &mut self.usernames.lock().await;
        if usernames.contains_key(&username) {
            return Err("Username already taken!");
        }
        let password_hash = password::hash_password(password)?;
        let id = self.user_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let user = User::new(id, username.to_string(), password_hash, username.to_string(), role);
        self.users.lock().await.insert(id, user.clone());
        usernames.insert(username, id);
        Ok(user)
    }
    pub async fn get(&self, id: UserId) -> Result<User, ErrType<'static>> {
        self.users.lock().await.get(&id).cloned().ok_or("User does not exist!")
    }
    pub async fn get_by_username(&self, username: &str) -> Result<User, ErrType<'static>> {
        let usernames = &self.usernames.lock().await;
        let user_id = usernames.get(username).ok_or("User does not exist!")?;
        self.get(*user_id).await
    }
    pub async fn update(&self, user: User) -> bool {
        let users = &mut self.users.lock().await;
        if users.contains_key(&user.id) {
            users.insert(user.id, user.clone());
            true
        } else {
            false
        }
    }
    pub async fn remove_user(&self, id: UserId) -> Option<User> {
        self.users.lock().await.remove(&id)
    }
    pub async fn clear_room(&self, room: &Room) -> () {
        let mut users = self.users.lock().await;
        room.players.clone().into_iter().for_each(|player| {
            if let Some(p) = player {
                let user = users.get_mut(&p.user_id);
                if let Some(user) = user{
                    user.current_room = None;
                }
            }
        });
    }
}