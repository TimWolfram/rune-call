use crate::model::game::{CreateRoomForm, Player, Room};
use crate::model::login::{LoginToken, Role};
use crate::repository::{UserRepository, RoomRepository};

use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket::State;

impl Room {
    fn create_from_form(id: usize, room_name: String, room_password: String , host_player: &Player) -> Room {
        Room {
            id,
            name: room_name,
            password: room_password,
            host_user_id: host_player.user_id,
            players: [Some(host_player.clone()),
                None,
                None,
                None],
            game: None,
            game_history: Vec::new(),
        }
    }
    pub fn add_player(&mut self, player: &Player, index: usize) -> bool {
        if index > 3 {
            return false;
        }
        if self.players[index].is_some() {
            return false;
        }
        self.players[index] = Some(player.clone());
        true
    }
    pub fn swap_player_seats(&mut self, player1_index: usize, player2_index: usize) {
        self.players.swap(player1_index, player2_index);
    }
}

// return a list of all rooms
#[get("/rooms", format = "json")]
pub async fn get_rooms(room_repo: &State<RoomRepository>) -> Json<Vec<Room>> {
    Json(room_repo.rooms.lock().await.values().cloned().collect())
}

// get room by id (join room -- requires room code)
#[get("/rooms/<id>")]
pub async fn get_room(id: usize, room_repo: &State<RoomRepository>) 
-> Option<Json<Room>> {
    if let Some(room) = room_repo.rooms.lock().await.get(&id) {
        return Some(Json(room.clone()));
    }
    None
}

#[put("/rooms/<room_id>/players", format = "json", data = "<swap>")]
pub async fn swap_player_seats<'a> (room_id: usize, 
                                    swap: Json<(usize, usize)>,
                                    room_repo: &State<RoomRepository>, 
                                    cookies: &'a CookieJar<'a>) 
-> Result<Json<Room>, &'static str> {
    //only host can swap player seats: check if logged in player is host of room
    LoginToken::from_cookies(cookies)?.user_id;
    let mut rooms = room_repo.get_room_by_host(host_user_id).await;
    let room = rooms.get_mut(&room_id).ok_or("Room not found!")?;
    room.swap_player_seats(player1_index, player2_index);
    Ok(Json(room.clone()))
}

// //delete room
#[delete("/rooms/<id>", format = "json")]
pub async fn delete_room<'a>(
    id: usize,
    room_repo: &State<RoomRepository>,
    user_repo: &State<UserRepository>,
    cookies: &CookieJar<'a>,
) -> (Status, &'a str){
    //get player cookies first; if cookie does not exist, we don't need to lock the mutex
    let user_token = LoginToken::from_cookies(cookies);
    if let Err(reason) = user_token {
        return (Status::Unauthorized, reason);
    }
    let user_id = user_token.unwrap().user_id;
    let host = user_repo
            .get(user_id).await
            .ok_or("User does not exist!");
    if let Err(reason) = host {
        return (Status::Unauthorized, reason);
    }
    let host = host.unwrap();
    
    let authorized = match host.role {
        Role::Admin => true,
        Role::Player => {
            user_id == host.id
        }
    };
    if !authorized {
        return (Status::Unauthorized, "User must be host or admin to delete room!");
    }

    match room_repo.rooms.lock().await.remove(&id) {
        None => (Status::NotFound, "User must be host or admin to delete room!"),
        Some(_) => (Status::Ok, "Succesfully deleted room!"),
    }
}
    
// create room
#[post("/rooms", format = "json", data = "<create_room_form>")]
pub async fn create_room<'a>(
    create_room_form: Json<CreateRoomForm>,
    room_repo: &State<RoomRepository>,
    user_repo: &State<UserRepository>,
    cookies: &'a CookieJar<'a>) 
-> (Status, Result<Json<Room>, &'a str>) {
    println!("create_room_form: {:?}", create_room_form);

    if create_room_form.name.len() < 3 {
        return (Status::BadRequest, Err("Room name must be at least 3 characters long!"));
    }
    //get currently logged in user
    let login_token = LoginToken::from_cookies(cookies);
    if let Err(e) = login_token {
        return (Status::Unauthorized, Err(e));
    }
    let user_id = login_token.unwrap().user_id;
    let rooms = room_repo.rooms.lock().await;
    if rooms.contains_key(&user_id) {
        return (Status::BadRequest, Err("Player is already hosting a room!"));
    }
    //find user
    let host_user = user_repo
        .get(user_id).await
        .ok_or("User does not exist!");

    if let Err(reason) = host_user {
        return (Status::Unauthorized, Err(reason));
    }
    let host_player = Player::from(host_user.unwrap());
    let room_id = room_repo.room_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let from_form = Room::create_from_form(room_id, create_room_form.name.clone(), create_room_form.password.clone(), &host_player);
    (Status::Ok, Ok(Json(from_form)))
}

//join room
// #[put("/rooms/<room_id>/players", format = "json")]
// pub fn put_room(room_id: usize, cookies: &rocket::http::CookieJar) -> Json<Option<Room>> {
//     cookies.get("token");

//     // Decode JWT token from cookie
//     let token_cookie = cookies.get_private("token");
//     // Decode JWT token
//     let token_message = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256));

//     let token = jsonwebtoken::decode::<jwt::PlayerToken>(x, &DecodingKey::from_secret(x), ).unwrap();
// }
