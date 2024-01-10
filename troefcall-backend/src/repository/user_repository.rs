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
        println!("Creating user repository with default admin account.");
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
impl Clone for UserRepository {
    fn clone(&self) -> Self {
        UserRepository {
            users: Mutex::new(self.users.try_lock().unwrap().clone()),
            usernames: Mutex::new(self.usernames.try_lock().unwrap().clone()),
            user_count: AtomicUsize::new(self.user_count.load(std::sync::atomic::Ordering::SeqCst)),
        }
    }
}

impl UserRepository{
    pub fn test_repo() -> Self {
        let test_user_amt = 15;

        println!("Creating user repository with test data.");
        // admin user
        let admin_user:(UserId, User) = (0, User {
                id: 0, 
                username: "admin".to_string(),
                password_hash: password::hash_password("adminpw!").unwrap(),
                nickname: "👍Admin👍".to_string(),
                role: Role::Admin,
                current_room: None,
            });
        let admin_user_nametuple: (String, UserId) = (admin_user.1.username.clone(), 0);

        let mut users = HashMap::from([admin_user]);
        let mut usernames: HashMap<String, UserId> = HashMap::from([admin_user_nametuple]);

        for i in 1..test_user_amt {
            let user = User::new(
                i,
                format!("user{}", i),
                password::hash_password("userpw!").unwrap(),
                format!("User {}", i),
                Role::Admin
            );
            usernames.insert(user.username.clone(), i);
            users.insert(i, user);
        }
        UserRepository {
            users: Mutex::new(users),
            usernames: Mutex::new(usernames),
            user_count: AtomicUsize::new(test_user_amt),
        }
    }
}

type Error<'a> = (Status, &'a str);

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