// controller for rooms; endpoints & logic
use crate::model::game::{CreateRoomForm, Player, Room};
use crate::model::login::{LoginToken, Role, LoginForm};
use crate::repository::{UserRepository, RoomRepository, GameRepository};

use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket::State;

impl Room {
    pub fn add_player(&mut self, player: Player, index: usize) -> Result<(), &'static str> {
        if index > 3 {
            return Err("Player index out of bounds!");
        }
        if self.players[index].is_some() {
            return Err("Player slot already taken!");
        }
        self.players[index] = Some(player.clone());
        return Ok(())
    }
    pub fn swap_player_seats(&mut self, player1_index: usize, player2_index: usize) {
        self.players.swap(player1_index, player2_index);
    }
} 
#[get("/rooms/page/<page>", format = "json", rank = 1)]
pub async fn get_rooms_paged(room_repo: &State<RoomRepository>, page: usize) -> Json<Vec<Room>> {
    const PAGE_SIZE: usize = 10;
    let start = page * PAGE_SIZE;
    Json(room_repo.get_rooms_paged(start, PAGE_SIZE).await)
}
#[get("/rooms/public/page/<page>", format = "json", rank = 2)]
pub async fn get_rooms_public_paged(room_repo: &State<RoomRepository>, page: usize) -> Json<Vec<Room>> {
    const PAGE_SIZE: usize = 10;
    let start = page * PAGE_SIZE;
    Json(room_repo.get_rooms_public_paged(start, PAGE_SIZE).await)
}
#[get("/rooms/<id>", format = "json")]
pub async fn get_room(id: usize, room_repo: &State<RoomRepository>) -> Option<Json<Room>> {
    if let Ok(room) = room_repo.get_room_by_id(id).await {
        return Some(Json(room.clone()));
    }
    None
}
#[post("/rooms", format = "json", data = "<create_room_form>")]
pub async fn create_room(
    create_room_form: Json<CreateRoomForm>,
    room_repo: &State<RoomRepository>,
    user_repo: &State<UserRepository>,
    cookies: &CookieJar<'_>) 
-> (Status, Result<Json<Room>, &'static str>) {
    let name = create_room_form.name.clone();
    if name.len() < 3 {
        return (Status::BadRequest, Err("Room name must be at least 3 characters long!"));
    }
    let password = create_room_form.password.clone();
    
    //get currently logged in user
    let login_token = LoginToken::try_refresh(cookies);
    if let Err(e) = login_token {
        return (Status::Unauthorized, Err(e));
    }
    let user_id = login_token.unwrap();
    let user = user_repo.get(user_id).await;
    
    if let Err(e) = user {
        return (Status::Unauthorized, Err(e));
    }
    let mut user = user.unwrap();
    //find user
    let room = room_repo.create_room(&mut user, name, password).await;
    match room {
        Err(reason) => (Status::BadRequest, Err(reason)),
        Ok(room) => {
            //update user
            user_repo.update(user).await;
            (Status::Ok, Ok(Json(room)))
        }
    }
}
#[put("/rooms/<room_id>/host/players", format = "json", data = "<swap>")]
pub async fn swap_player_seats<'a> (
    swap: Json<(usize, usize)>,
    room_id: usize,
    room_repo: &State<RoomRepository>,
    cookies: &'a CookieJar<'a>)
-> Result<Json<Room>, &'a str> {
    //only host can swap player seats: check if logged in player is host of room
    let host_user_id: usize = LoginToken::try_refresh(cookies)?;
    let mut room: Room = room_repo.get_room_by_id(room_id).await?;
    if room.host_id != host_user_id { return Err("Only the host can swap player seats!"); }
    let swap: (usize, usize) = swap.into_inner();
    room.swap_player_seats(swap.0, swap.1);
    Ok(Json(room.clone()))
}
#[delete("/rooms/<room_id>", format = "json")]
pub async fn delete_room<'a>(
    room_id: usize,
    room_repo: &State<RoomRepository>,
    user_repo: &State<UserRepository>,
    cookies: &CookieJar<'a>) 
-> (Status, &'a str){
    //get player cookies first; if cookie does not exist, we don't need to lock the mutex
    let user_token = LoginToken::try_refresh(cookies);
    if let Err(reason) = user_token {
        return (Status::Unauthorized, reason);
    }
    let user_id = user_token.unwrap();
    let host = user_repo
            .get(user_id).await;
    if let Err(e) = host {
        return (Status::Unauthorized, e);
    }
    let room = room_repo.get_room_by_id(room_id).await;
    if let Err(e) = room {
        return (Status::NotFound, e);
    }
    let room = room.unwrap();
    let mut host = host.unwrap();
    let authorized = match host.role {
        Role::Admin => true,
        Role::Player => user_id == host.id
    };
    if !authorized {
        return (Status::Unauthorized, "User must be host or admin to delete this room!");
    }
    user_repo.clear_room(&room).await;
    room_repo.delete_room(&room_id).await;
    host.current_room = None;
    user_repo.update(host).await;
    (Status::Ok, "Succesfully deleted room!")
}
#[post("/rooms/<room_id>/players/<player_index>", format = "json", data = "<player>")]
pub async fn join_room<'a>(
    room_id: usize,
    user_repo: &State<UserRepository>,
    room_repo: &State<RoomRepository>,
    player_index: usize,
    player: Option<Json<LoginForm<'_>>>,
    cookies: &CookieJar<'a> ) 
-> Result<Json<Room>, &'a str> {
    let user_id = LoginToken::try_refresh(cookies).unwrap();
    let mut user = user_repo.get(user_id).await?;
    if user.role == Role::Admin {
        if let Some(player) = player {
            // admin can join as any other user
            user = user_repo.get_by_username(player.username).await?;
        }
    }
    if user.current_room.is_some() {
        return Err("User is already in a room! Leave the room before joining another one!");
    }
    user.current_room = Some(room_id);
    let player = Player::from(&user);
    let room = &mut room_repo.get_room_by_id(room_id).await?;
    room.add_player(player, player_index)?;
    Ok(Json(room.clone()))
}
// #[delete("/rooms/<room_id>/players/<player_id>", format = "json")]
// pub async fn kick_player<'a>(
//     room_id: usize,
//     user_id: usize,
//     user_repo: &State<UserRepository>,
//     room_repo: &State<RoomRepository>,
//     game_repo: &State<GameRepository>,
//     cookies: &CookieJar<'a>)
// -> Result<Json<Room>, &'a str> {
//     let user = user_repo.get(user_id).await?;
//     if user.current_room != Some(room_id) {
//         return Err("User is not in room!");
//     }
//     let mut room = room_repo.get_room_by_id(room_id).await.ok_or("Room not found!")?;
//     leave_room(room_id, user_repo, room_repo, game_repo, cookies).await?;
//     Err("User is not in room!")
// }
#[delete("/rooms/<room_id>/players", format = "json")]
pub async fn leave_room<'a>(
    room_id: usize,
    user_repo: &State<UserRepository>,
    room_repo: &State<RoomRepository>,
    game_repo: &State<GameRepository>,
    cookies: &CookieJar<'a>) 
-> Result<(), &'a str> {
    let user_id = LoginToken::try_refresh(cookies).unwrap();
    let user = user_repo.get(user_id).await?;
    if user.current_room != Some(room_id) {
        return Err("User is not in room!");
    }
    // check if room game is in progress
    let game = game_repo.get_game_from_room(room_id).await;
    if let Ok(game) = game {
        if game.is_in_progress() {
            return Err("Cannot leave room while game is in progress! Finish or forfeit the game first!");
        }
    }
    
    let mut room = room_repo.get_room_by_id(room_id).await?;
    let mut host_found = false;
    // check if user is host
    if room_repo.user_is_host(user_id, room_id).await {
        // if user is host, transfer ownership to another player
        for i in 0..4 {
            if let Some(p) = &room.players[i] {
                if p.user_id != user_id {
                    room.host_id = p.user_id;
                    room_repo.transfer_host(user_id, p.user_id).await;
                    host_found = true;
                    break;
                }
            }
        }
        if !host_found {
            // if no other players are in the room, delete the room
            room_repo.delete_room(&room_id).await;
            return Ok(());
        }
    }
    for i in 0..4 {
        if let Some(p) = &room.players[i] {
            if p.user_id == user_id {
                room.players[i] = None;
                return Ok(());
            }
        }
    }
    room_repo.update_room(room).await;
    Err("User is not in room!")
}
