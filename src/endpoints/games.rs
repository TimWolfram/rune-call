use crate::model::Game;
use crate::repository::RoomRepository;

use rocket::serde::json::Json;
use rocket::State;

//get game history for room; NYI
#[get("/rooms/<id>/games")]
pub fn get_games(id: usize, room_repo: &State<RoomRepository>) -> Json<Vec<Game>> {
    Json(room_repo.rooms[id].played_games.clone())
}


//get game by id (join game -- requires room code)
#[get("/rooms/<id>/games/current")]
pub fn get_game(id: usize, room_repo: &State<RoomRepository>) -> Game {
    room_repo.rooms[id].played_games.last().unwrap().clone()
}

//start game
#[post("/rooms/<id>/games")]
pub fn create_game(id: usize, room_repo: &State<RoomRepository>) -> Result<Game, &str> {
    let room = room_repo.rooms.get_mut(id).expect("Room not found!");
    if room.current_players.len() < 3 {
        return Err("Not enough players!");
    }
    let game = Game::start(&room);
    room_repo.rooms.played_games.push(game);
    Ok(room.played_games.last().unwrap().clone())
}