use std::{collections::HashMap, sync::atomic::AtomicUsize};
use rocket::{tokio::sync::Mutex, http::Status};
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

impl UserRepository{
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
                (1, User::new(
                    1,
                    "user1".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user1".to_string(),
                    Role::Admin)
                ),
                (2, User::new(
                    2,
                    "user2".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user2".to_string(),
                    Role::Admin)
                ),
                (3, User::new(
                    3,
                    "user3".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user3".to_string(),
                    Role::Admin)
                ),
                (4, User::new(
                    4,
                    "user4".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user4".to_string(),
                    Role::Admin)
                ),
                (5, User::new(
                    5,
                    "user5".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user5".to_string(),
                    Role::Admin)
                ),
                (6, User::new(
                    6,
                    "user6".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user6".to_string(),
                    Role::Admin)
                ),
                (7, User::new(
                    7,
                    "user7".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user7".to_string(),
                    Role::Admin)
                ),
                (8, User::new(
                    8,
                    "user8".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user8".to_string(),
                    Role::Admin)
                ),
                (9, User::new(
                    9,
                    "user9".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user9".to_string(),
                    Role::Admin)
                ),
                (10, User::new(
                    10,
                    "user10".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user10".to_string(),
                    Role::Admin)
                ),
                (11, User::new(
                    11,
                    "user11".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user11".to_string(),
                    Role::Admin)
                ),
                (12, User::new(
                    12,
                    "user12".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user12".to_string(),
                    Role::Admin)
                ),
                (13, User::new(
                    13,
                    "user13".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user13".to_string(),
                    Role::Admin)
                ),
                (14, User::new(
                    14,
                    "user14".to_string(),
                    password::hash_password("userpw!").unwrap(),
                    "user14".to_string(),
                    Role::Admin)
                ),
            ])),
            usernames: Mutex::new(HashMap::from([
                ("admin".to_string(), 0),
                ("user1".to_string(), 1),
                ("user2".to_string(), 2),
                ("user3".to_string(), 3),
                ("user4".to_string(), 4),
                ("user5".to_string(), 5),
                ("user6".to_string(), 6),
                ("user7".to_string(), 7),
                ("user8".to_string(), 8),
                ("user9".to_string(), 9),
                ("user10".to_string(), 10),
                ("user11".to_string(), 11),
                ("user12".to_string(), 12),
                ("user13".to_string(), 13),
                ("user14".to_string(), 14),
            ])),
            user_count: AtomicUsize::new(15),
        }
    }

}
type Error<'a> = (Status, &'a str);
type EndpointResult<'a, T> = Result<T, Error<'a>>;

impl UserRepository {
    pub async fn create_user<'a>(&'a self, username: &str, password: &'a str, role: Role) -> Result<User, Error<'a>> {
        let username = username.to_string();
        let usernames = &mut self.usernames.lock().await;
        if usernames.contains_key(&username) {
            return Err((Status::BadRequest, "Username already taken!"));
        }
        let password_hash = password::hash_password(password)?;
        let id = self.user_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let user = User::new(id, username.to_string(), password_hash, username.to_string(), role);
        self.users.lock().await.insert(id, user.clone());
        usernames.insert(username, id);
        Ok(user)
    }
    pub async fn get(&self, id: UserId) -> Result<User, Error<'static>> {
        self.users.lock().await.get(&id).cloned().ok_or((Status::Unauthorized, "User does not exist!"))
    }
    pub async fn get_by_username(&self, username: &str) -> Result<User, Error<'static>> {
        let usernames = &self.usernames.lock().await;
        let user_id = usernames.get(username).ok_or((Status::Unauthorized, "User does not exist!"))?;
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