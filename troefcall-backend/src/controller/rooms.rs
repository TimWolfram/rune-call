// controller for rooms; endpoints & logic
use crate::model::game::{CreateRoomForm, Player, Room};
use crate::model::login::{LoginToken, Role, LoginForm};
use crate::repository::{UserRepository, RoomRepository, GameRepository};

use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket::State;
type Error<'a> = (Status, &'a str);

impl Room {
    pub fn add_player<'a>(&mut self, player: Player, index: usize) -> Result<(), Error<'a>> {
        if index > 3 {
            return Err((Status::InternalServerError, "Player index out of bounds!"));
        }
        if self.players[index].is_some() {
            return Err((Status::BadRequest, "Player slot already taken!"));
        }
        self.players[index] = Some(player.clone());
        return Ok(())
    }

    pub fn swap_player_seats(&mut self, player1_index: usize, player2_index: usize) {
        self.players.swap(player1_index, player2_index);
    }
} 

#[get("/page/<page>", format = "json", rank = 1)] //without rank = 1 the compiler complains about ambiguous routes between this (GET /rooms/page/<page>) and game (GET /rooms/<room_id>/game).. not sure why (bug in rocket crate?)
pub async fn get_rooms_paged(room_repo: &State<RoomRepository>, page: usize) -> Json<Vec<Room>> {
    const PAGE_SIZE: usize = 10;
    let start = page * PAGE_SIZE;
    Json(room_repo.get_rooms_paged(start, PAGE_SIZE).await)
}

#[get("/page/<page>?public=true", format = "json")]
pub async fn get_rooms_public_paged(room_repo: &State<RoomRepository>, page: usize) 
-> Json<Vec<Room>> {
    const PAGE_SIZE: usize = 10;
    let start = page * PAGE_SIZE;
    Json(room_repo.get_rooms_public_paged(start, PAGE_SIZE).await)
}

#[get("/<id>", format = "json")]
pub async fn get_room(id: usize, room_repo: &State<RoomRepository>) 
-> Option<Json<Room>> {
    if let Ok(room) = room_repo.get_room_by_id(id).await {
        return Some(Json(room.clone()));
    }
    None
}

#[post("/", data = "<create_room_form>", format = "json")]
pub async fn create_room<'a>(
    create_room_form: Json<CreateRoomForm>,
    room_repo: &'a State<RoomRepository>,
    user_repo: &'a State<UserRepository>,
    cookies: &'a CookieJar<'_>) 
-> Result<Json<Room>, Error<'a>> {
    let name = create_room_form.name.clone();
    if name.len() < 3 {
        return Err((Status::BadRequest, "Room name must be at least 3 characters long!"));
    }
    let password = create_room_form.password.clone();
    
    //get currently logged in user
    let user_id = LoginToken::refresh_jwt(cookies)?;
    let mut user = user_repo.get(user_id).await?;
    //find user
    let room = room_repo.create_room(&mut user, name, password).await?;
    user_repo.update(user).await;
    Ok(Json(room))
}

#[put("/<room_id>/host/players", data = "<swap>", format = "json")]
pub async fn swap_player_seats<'a> (
    swap: Json<(usize, usize)>,
    room_id: usize,
    room_repo: &'a State<RoomRepository>,
    cookies: &'a CookieJar<'a>)
-> Result<Json<Room>, Error<'a>> {
    //only host can swap player seats: check if logged in player is host of room
    let host_user_id: usize = LoginToken::refresh_jwt(cookies)?;
    let mut room: Room = room_repo.get_room_by_id(room_id).await?;
    if room.host_id != host_user_id { return Err((Status::Unauthorized, "Only the host can swap player seats!")); }
    let swap: (usize, usize) = swap.into_inner();
    room.swap_player_seats(swap.0, swap.1);
    Ok(Json(room.clone()))
}

#[delete("/<room_id>", format = "json")]
pub async fn delete_room<'a>(
    room_id: usize,
    room_repo: &'a State<RoomRepository>,
    user_repo: &'a State<UserRepository>,
    cookies: &'a CookieJar<'a>) 
-> Result<(), Error<'a>> {
    //no need to lock mutex if user is not logged in
    let user_id = LoginToken::refresh_jwt(cookies)?;
    
    let mut host = user_repo
            .get(user_id).await?;
    let room = room_repo.get_room_by_id(room_id).await?;
    let authorized = match host.role {
        Role::Admin => true,
        Role::Player => user_id == host.id
    };
    if !authorized {
        return Err((Status::Forbidden, "User must be host or admin to delete this room!"));
    }
    user_repo.clear_room(&room).await;
    room_repo.delete_room(&room_id).await;
    host.current_room = None;
    user_repo.update(host).await;
    Ok(())
}

#[post("/<room_id>/players/<player_index>", data = "<player>", format = "json")]
pub async fn join_room<'a>(
    room_id: usize,
    user_repo: &'a State<UserRepository>,
    room_repo: &'a State<RoomRepository>,
    player_index: usize,
    player: Option<Json<LoginForm<'_>>>,
    cookies: &'a CookieJar<'a> ) 
-> Result<Json<Room>, Error<'a>> {
    let user_id = LoginToken::refresh_jwt(cookies).unwrap();
    let mut user = user_repo.get(user_id).await?;
    if user.role == Role::Admin {
        if let Some(player) = player {
            // admin can join as any other user
            user = user_repo.get_by_username(player.username).await?;
        }
    }
    if user.current_room.is_some() {
        return Err((Status::Conflict, "User is already in a room! Leave the room before joining another one!"));
    }
    user.current_room = Some(room_id);
    let player = Player::from(&user);
    let room = &mut room_repo.get_room_by_id(room_id).await?;
    room.add_player(player, player_index)?;
    Ok(Json(room.clone()))
}

// #[delete("/<room_id>/players/<player_id>", format = "json")]
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

#[delete("/<room_id>/players", format = "json")]
pub async fn leave_room<'a>(
    room_id: usize,
    user_repo: &'a State<UserRepository>,
    room_repo: &'a State<RoomRepository>,
    game_repo: &'a State<GameRepository>,
    cookies: &CookieJar<'a>) 
-> Result<(), Error<'a>> {
    let user_id = LoginToken::refresh_jwt(cookies).unwrap();
    let user = user_repo.get(user_id).await?;
    if user.current_room != Some(room_id) {
        return Err((Status::Unauthorized, "User is not in room!"));
    }
    // check if room game is in progress
    let game = game_repo.get_game_from_room(room_id).await;
    if let Ok(game) = game {
        if game.is_in_progress() {
            return Err((Status::Conflict, "Cannot leave room while game is in progress! Finish or forfeit the game first!"));
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
    else {
        for i in 0..4 {
            if let Some(p) = &room.players[i] {
                if p.user_id == user_id {
                    room.players[i] = None;
                }
            }
        }
    }
    room_repo.update_room(room).await;
    return Ok(());
}
