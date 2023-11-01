use crate::model::game::{Room, Game, Card, Player, GameState};
use crate::model::login::{UserId, LoginToken};
use crate::repository::{RoomRepository, UserRepository, GameRepository};

use rocket::http::CookieJar;
use rocket::State;
use rocket::serde::json::Json;

impl Game {
    /// Get the order of players for a new game.
    fn player_order(room: &Room, last_game: Option<Game>) -> Result<[Player;4], &'static str> {
        // check if room has enough players
        let players = room.players.clone();
        for player in &players {
            if let None = player {
                return Err("Room does not have enough players!");
            }
        }
        // convert players to array
        let mut players: [Player; 4] = players.into_iter()
            .map(|player| player.unwrap())
            .collect::<Vec<Player>>().try_into()
            .or(Err("Room should have 4 players!"))?;
        // check who the next starting player should be
        enum StartingPlayer{ Last, Next, Random }
        let starting_player = match last_game {
            None => StartingPlayer::Random,
            Some(game) => {
                if let GameState::Finished{winners} = &game.game_state {
                    // check if starting player won
                    let last_starting = &game.players[0];
                    match winners.contains(last_starting) {
                        true => StartingPlayer::Last,
                        false => StartingPlayer::Next,
                    }
                } else {
                    return Err("Last game is not finished!");
                }
            }
        };
        
        // rotate players so that starting player is first
        match starting_player {
            StartingPlayer::Last => Ok(players),
            StartingPlayer::Next => {
                players.rotate_left(1);
                Ok(players)
            },
            StartingPlayer::Random => {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let len = players.len();
                players.rotate_left(rng.gen_range(0..len));
                Ok(players)
            }
        }
    }
    /// Create a new game from a room.
    pub fn create(room: &mut Room, last_game: Option<Game>) -> Result<Game, &'static str> {
        let mut players = Game::player_order(room, last_game)?;
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
        Ok(game)
    }
}

//get game by id (join game -- requires room code)
#[get("/rooms/<room_id>/game")]
pub async fn get_game<'a>( room_id: usize, 
    room_repo: &'a State<RoomRepository>, 
    game_repo: &'a State<GameRepository> )
-> Result<Json<Game>, &'a str> {
    let room = room_repo.get_room_by_id(room_id).await
        .ok_or("Room not found!")?;
    let game = game_repo.get_game_from_room(room.id).await
        .ok_or("Game not found!")?;
    Ok(Json(game.clone()))
}

// create(start) game
#[post("/rooms/<id>/game", format = "json")]
pub async fn create_game(id: UserId, room_repo: &State<RoomRepository>, user_repo: &State<UserRepository>, game_repo: &State<GameRepository>, cookies: &CookieJar<'_>,) 
-> Result<Json<Game>, &'static str> {
    // first check if player is logged in before trying to create game; if not, no need to lock mutex
    let player_token = LoginToken::from_cookies(cookies)?;
    if user_repo.get(player_token.user_id).await.is_none() {
        return Err("User does not exist!");
    }
    // get room
    let room = room_repo.get_room_by_id(id).await.ok_or("Room not found!")?; 
    if room.players.len() < 3 {
        return Err("Not enough players in room to start game! You need at least 3 players.");
    }
    if game_repo.get_game_from_room(room.id).await.is_some() {
        return Err("Game already started!");
    }
    if player_token.user_id != room.host_id {
        return Err("Only the host can start the game!")
    }
    let game = game_repo.create_game(room).await?;
    Ok(Json(game.clone()))
}