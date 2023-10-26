use std::collections::HashMap;

use crate::model::{Game, Player, Room, PlayerTeams, Round, PlayerToken};
use crate::repository::{PlayerRepository, RoomRepository};

use rocket::http::CookieJar;
use rocket::State;
use rocket::serde::json::Json;


impl Game{
    pub fn create(room: &Room) -> Game {
        let mut teams:PlayerTeams = HashMap::new();
        for (team_id, team) in room.player_teams.iter() {
            let mut player_ids:Vec<usize> = Vec::new();
            for player_id in team {
                player_ids.push(player_id.clone());
            }
            teams.insert(team_id.clone(), player_ids);
        }
        Game {
            current_round: Round::new(),
            player_order: room.players.clone(),
            teams: teams,
            played_rounds: Vec::new(),
            player_cards: HashMap::new(),
            blessed_rune: None,
        }
    }
    pub fn new(players: Vec<Player>, player_teams: PlayerTeams) -> Game {
        let mut teams = player_teams.clone();
        Game {
            current_round: Round::new(),
            player_order: players,
            teams: teams,
            played_rounds: Vec::new(),
            player_cards: HashMap::new(),
            blessed_rune: None,
        }
    }
}
//get game by id (join game -- requires room code)
#[get("/rooms/<room_id>/game")]
pub async fn get_game(room_id: usize, room_repo: &State<RoomRepository>) -> Result<Json<Game>, &str> {
    let games = room_repo.games.lock().await;
    match games.get(&room_id) {
        None => {
            //check if room exists
            let rooms = &room_repo.rooms.lock().await;
            if !rooms.contains_key(&room_id) {
                return Err("Room does not exist!");
            }
            Err("Game not found!")
        }
        Some(game) => Ok(Json(game.clone())),
    }
}

// create(start) game
#[post("/rooms/<id>/game", format = "json")]
pub async fn create_game<'a>(
    id: usize,
    room_repo: &State<RoomRepository>,
    player_repo: &State<PlayerRepository>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Game>, String> { //has to return String instead of &str because for some reason you cannot format a &str
    // get room
    let rooms = &room_repo.rooms.lock().await;
    let room = rooms.get(&id);
    if room.is_none() {
        return Err("Room not found!".to_string());
    }

    // room exists
    let room = room.unwrap();
    if room.player_teams.len() < 3 {
        return Err("Not enough players in room!".to_string());
    }

    let games = &room_repo.games.lock().await;
    // create game
    if games.contains_key(&id) {
        return Err("Game already started!".to_string());
    }
    
    let player_token = PlayerToken::from_cookies(cookies);
    if let Err(a) = player_token {
        return Err(a.to_string())
    }
    let player_token = player_token.unwrap();
    if player_token.player_id != room.host_player_id {
        return Err("Only the host can start the game!".to_string())
    }

    // get players from repository
    let players = &player_repo.players.lock().await;
    let player_teams = room.player_teams.clone();
    let mut not_found_players: Vec<&str> = Vec::new();
    //check if all players exist in repo
    player_teams.into_iter().for_each(|(_team_id, teams)| {
        for player_id in teams {
            let player = players.get(&player_id);
            if player.is_none() {
                // check if player id at least exists in room player list
                // if not, there is a programming error in the front end application
                let player = room.players.into_iter().find(|(id, _player)| *id == player_id);
                if player.is_none() {
                    panic!("A player id is not found in room player list! All player ids in team must be valid. This means the front end application contains an error.");
                }
                let player = player.unwrap();
                not_found_players.push(&player.name);
            }
        }
    });
    //players not found
    if not_found_players.len() > 0 {
        let mut error_message = String::new();
        if not_found_players.len() == 1 {
            error_message.push_str(format!("Player not found: {}", not_found_players[0]).as_str());
        } else {
            error_message.push_str("Players not found:");
            error_message.push_str(not_found_players.join(", ").as_str());
        }
        return Err(error_message);
    }
    
    let game = Game::create(room);
    Ok(Json(game))
}