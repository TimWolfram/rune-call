use crate::model::game::{Card, EndGameReason, Game, GameState, Player, Room, RoomId, Suit, PlayedCard};
use crate::model::login::{LoginToken, Role, UserId};
use crate::repository::{GameRepository, RoomRepository, UserRepository};

use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::State;

impl Game {
    /// Get the order of players for a new game.
    fn player_order(room: &Room, last_game: Option<Game>) -> Result<[Player; 4], &'static str> {
        // check if room has enough players
        let players = room.players.clone();
        for player in &players {
            if let None = player {
                return Err("Room does not have enough players!");
            }
        }
        // convert players to array
        let mut players: [Player; 4] = players
            .into_iter()
            .map(|player| player.unwrap())
            .collect::<Vec<Player>>()
            .try_into()
            .or(Err("Room should have 4 players!"))?;
        // check who the next starting player should be
        enum StartingPlayer {
            Last,
            Next,
            Random,
        }
        let starting_player = match last_game {
            None => StartingPlayer::Random,
            Some(game) => {
                if let GameState::Finished { winners, reason: _ } = game.game_state {
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
            }
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
            game_state: GameState::Starting {
                remaining_deck: deck,
            },
        };
        Ok(game)
    }
    pub fn valid_cards(player: Player, suit: Suit) -> Vec<Card> {
        let mut valid_cards = player.current_cards.clone();
        // if player has suit, that suit must be played
        if valid_cards.iter().any(|card| card.suit == suit) {
            valid_cards.retain(|card| card.suit == suit);
        }
        valid_cards
    }
}

//get game by id (join game -- requires room code)
#[get("/rooms/<room_id>/game")]
pub async fn get_game<'a>(
    room_id: usize,
    room_repo: &'a State<RoomRepository>,
    game_repo: &'a State<GameRepository>,
) -> Result<Json<Game>, &'a str> {
    let room = room_repo
        .get_room_by_id(room_id)
        .await
        .ok_or("Room not found!")?;
    let game = game_repo
        .get_game_from_room(room.id)
        .await
        .ok_or("Game not found!")?;
    Ok(Json(game.clone()))
}
// create(start) game
#[post("/rooms/<id>/game")]
pub async fn create_game(
    id: UserId,
    room_repo: &State<RoomRepository>,
    user_repo: &State<UserRepository>,
    game_repo: &State<GameRepository>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Game>, &'static str> {
    // first check if player is logged in before trying to create game; if not, no need to lock mutex
    let player_token = LoginToken::from_cookies(cookies)?;
    if user_repo.get(player_token.user_id).await.is_none() {
        return Err("User does not exist!");
    }
    // get room
    let room = room_repo
        .get_room_by_id(id)
        .await
        .ok_or("Room not found!")?;
    if room.players.len() < 3 {
        return Err("Not enough players in room to start game! You need at least 3 players.");
    }
    if game_repo.get_game_from_room(room.id).await.is_some() {
        return Err("Game already started!");
    }
    if player_token.user_id != room.host_id {
        return Err("Only the host can start the game!");
    }
    let game = game_repo.create_game(room).await?;
    Ok(Json(game.clone()))
}

#[put("/rooms/<room_id>/game", data = "<card>")]
pub async fn play_card(
    room_id: RoomId,
    room_repo: &State<RoomRepository>,
    game_repo: &State<GameRepository>,
    user_repo: &State<UserRepository>,
    card: Json<Card>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Game>, &'static str> {
    let player_token = LoginToken::from_cookies(cookies)?;
    let room = room_repo
        .get_room_by_id(room_id)
        .await
        .ok_or("Room not found!")?;
    let mut game = game_repo
        .get_game_from_room(room_id)
        .await
        .ok_or("Game not found!")?;
    let card = card.into_inner();
    let user_id = player_token.user_id;
    if let GameState::Playing { ref mut current_round, tjall: _, } = game.game_state {
        let user = user_repo.get(user_id).await.ok_or("User not found!")?;

        // check if it is player's turn (or player is admin)
        let player_turn = current_round.played_cards.len();
        let index = if let Role::Admin = user.role {
            // user is admin; allow them to play card as if they are current player
            Some(player_turn)
        } else {
            // user is not admin; check if they are current player
            room.players.iter().position(|room_player| {
                room_player.as_ref().map_or(false, |p| p.user_id == user_id)
            })
        }.ok_or("You are not a player in this room!")?;
        if index != player_turn {
            return Err("It is not your turn!");
        }

        // check if player has card
        let player = &game.players[index];
        if !player.current_cards.contains(&card) {
            return Err("You do not have this card!");
        }
        // check if card is valid
        let round_opening_card = current_round.played_cards[0].card.clone();
        let valid_cards = Game::valid_cards(player.clone(), round_opening_card.suit);
        if !valid_cards.contains(&card) {
            return Err(
                "You cannot play this card! If you have a card of the same suit, you must play it.",
            );
        }
        // play card
        let played_card = PlayedCard {
            player: player.clone(),
            card: card.clone(),
        };
        current_round.played_cards.push(played_card);
        // remove card from player's hand
        game.players[index].current_cards.retain(|c| *c != card);
    } else {
        return Err("Game is not in progress!");
    }
    Ok(Json(game.clone()))
}

#[delete("/rooms/<room_id>/game", format = "json")]
pub async fn forfeit(
    room_id: RoomId,
    room_repo: &State<RoomRepository>,
    game_repo: &State<GameRepository>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Game>, &'static str> {
    let player_token = LoginToken::from_cookies(cookies)?;
    let room = room_repo
        .get_room_by_id(room_id)
        .await
        .ok_or("Room not found!")?;
    let mut game = game_repo
        .get_game_from_room(room_id)
        .await
        .ok_or("Game not found!")?;
    // determine winners based on if index is odd or even
    let index = room
        .players
        .iter()
        .position(|p| {
            p.as_ref()
                .map_or(false, |p| p.user_id == player_token.user_id)
        })
        .ok_or("You are not a player in this room!")?;
    let winners = match index % 2 {
        0 =>
        // even
        {
            [game.players[1].clone(), game.players[3].clone()]
        }
        1 =>
        // odd
        {
            [game.players[0].clone(), game.players[2].clone()]
        }
        _ => unreachable!(),
    };
    //game is updated instead of actually deleted, because we need to remember last game's winners
    game.game_state = GameState::Finished {
        winners,
        reason: EndGameReason::Forfeit {
            forfeiting_player: game.players[index].clone(),
        },
    };
    let success = game_repo.update_game(room.id, game.clone()).await;
    match success {
        true => Ok(Json(game)),
        false => Err("Game not found!"),
    }
}
