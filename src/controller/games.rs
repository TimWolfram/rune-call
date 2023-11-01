use core::panic;

use crate::model::game::{Room, Game, Card, Player, GameState};
use crate::model::login::{UserId, LoginToken};
use crate::repository::{RoomRepository, UserRepository, GameRepository};

use rocket::http::CookieJar;
use rocket::State;
use rocket::serde::json::Json;

impl Game{
    fn start(room: &mut Room) -> Result<Game, &'static str> {
        // check if current game is finished
        if let Some(game) = &room.game {
            match &game.game_state {
                GameState::Finished{..} => {
                    // if game is finished, add it to game history
                    room.game_history.push(game.clone());
                }
                _ => return Err("Current game is not finished!"),
            }
        }

        // check if room has enough players
        for player in &room.players {
            assert!(player.is_some(), "Room should have 4 players!");
        }

        // find starting player
        let starting_player = match room.game_history.last(){
            Some(game) => {
                //get last game's starting player
                let last_starting = game.players[0].clone();
                // if starting player won, they start again
                if let GameState::Finished{winners} = game.game_state.clone()     {
                    if winners.contains(&last_starting) { 
                        last_starting
                    } else {
                        // if starting player lost, next player starts
                        let mut next_player_index = 0;
                        for (i, player) in game.players.iter().enumerate() {
                            if player.user_id == last_starting.user_id {
                                next_player_index = i + 1;
                            }
                        }
                        game.players[next_player_index].clone()
                    }
                } else {
                    panic!("Last game should be finished!");
                }
            },
            None => {
                //no game history: random starting player
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let random_index = rng.gen_range(0..room.players.len());
                room.players[random_index].clone().unwrap()
            }
        };
        let mut start_player_index = 0;
        
        let mut players: Vec<Player> = room
            .players
            .iter().enumerate()
            .map(|(i, player)|
        {
            match player{
                Some(player) => {
                    let player = player.clone();
                    if player == starting_player {
                        start_player_index = i;
                    }
                    player
                }
                None => panic!("Room does not have enough players!"),
            }
        })
        .collect();

        // rotate players so that starting player is first
        players.rotate_left(start_player_index);

        //draw 5 cards for each player
        let mut deck: Vec<Card> = Card::generate_deck();
        for player in &mut players {
            player.current_cards = deck.drain(0..5).collect();
        }
        let players: [Player; 4] = players.try_into().unwrap();
        let game = Game {
            players,
            played_rounds: Vec::new(),
            game_state: GameState::Starting{remaining_deck: deck},
        };
        room.game = Some(game.clone());
        Ok(game)
    }
}

//get game by id (join game -- requires room code)
#[get("/rooms/<room_id>/game")]
pub async fn get_game( room_id: usize, 
    room_repo: &'_ State<RoomRepository>, 
    game_repo: &'_ State<GameRepository> )
-> Result<Json<Game>, &'static str> {
    let room = room_repo.get_room_by_id(&room_id).await;
    match room {
        None => {
            return Err("Room does not exist!");
        }
        Some(room) => {
            if room.game.is_none() {
                return Err("Game not found!");
            }
            Ok(Json(room.game.clone().unwrap()))
        }
    }
}

// create(start) game
#[post("/rooms/<id>/game", format = "json")]
pub async fn create_game<'a>(id: UserId, room_repo: &State<RoomRepository>, user_repo: &State<UserRepository>, cookies: &CookieJar<'_>,) 
-> Result<Json<Game>, &'a str> {
    // first check if player is logged in before trying to create game; if not, no need to lock mutex
    let player_token = LoginToken::from_cookies(cookies)?;
    if user_repo.get(player_token.user_id).await.is_none() {
        return Err("User does not exist!");
    }
    // get room
    let rooms = &room_repo.rooms.lock().await;
    let room = rooms.get(&id).ok_or_else(|| "Room not found!")?; 
    if room.players.len() < 3 {
        return Err("Not enough players in room to start game! You need at least 3 players.");
    }
    if room.game.is_some() {
        return Err("Game already started!");
    }
    if player_token.user_id != room.host_user_id {
        return Err("Only the host can start the game!")
    }
    
    let game = Game::start(&mut room.clone())?;
    Ok(Json(game))
}