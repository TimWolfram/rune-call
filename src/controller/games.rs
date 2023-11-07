use crate::model::game::{
    Card, EndGameReason, Game, GameState, Player, Room, RoomId, Suit, Round,
};
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
    /// Get the valid cards that a player can play.
    pub fn valid_cards(player: &Player, suit: Suit) -> Vec<Card> {
        let mut valid_cards = player.current_cards.clone();
        // if player has suit, that suit must be played
        if valid_cards.iter().any(|card| card.suit == suit) {
            valid_cards.retain(|card| card.suit == suit);
        }
        valid_cards
    }
    /// Check if game is in progress.
    /// Returns true if game state is not `Finished`.
    pub fn is_in_progress(&self) -> bool {
        if let GameState::Finished { .. } = self.game_state {
            false
        } else {
            true
        }
    }
    /// Pick a tjall suit aka trump aka "troef".
    /// This starts the game; changing its state from `Starting` to `Playing`.
    pub fn pick_tjall_and_start(&mut self, suit: Suit) -> Result<(), &'static str> {
        if let GameState::Starting { remaining_deck: _ } = &mut self.game_state {
            self.game_state = GameState::Playing {
                current_round: Round {
                    played_cards: Vec::new(),
                    starting_player_index: 0,
                    round_winner: None,
                },
                tjall: suit,
            };
            Ok(())
        } else {
            Err("Game is not starting!")
        }
    }
    /// Play a card.
    pub fn play_card(&mut self, card: Card, index: usize) -> Result<(), &'static str> {
        // check if player has card
        let player = &self.players[index];
        if !player.current_cards.contains(&card) {
            return Err("You do not have this card!");
        }
        if let GameState::Playing { ref mut current_round, tjall } = self.game_state {
            let round_opening_card = &current_round.played_cards[0];
            let valid_cards = Game::valid_cards(player, round_opening_card.suit);
            if !valid_cards.contains(&card) {
                return Err("You cannot play this card! If you have a card of the same suit, you must play it.");
            }
            current_round.played_cards.push(card.clone());
            // remove card from player's hand
            self.players[index].current_cards.retain(|c| c.clone() != card);
            // check if round is finished
            if current_round.played_cards.len() == 4 {
                // determine winner
                let mut winning_card = &current_round.played_cards[0];
                for played_card in &current_round.played_cards {
                    winning_card = Card::compare(winning_card, played_card, tjall);
                }
            }
            Ok(())
        } else {
            return Err("Game is not in progress!")
        }
    }
}

impl Card {
    pub fn compare<'a>(highest: &'a Card, new: &'a Card, tjall: Suit) -> &'a Card {
        match highest.suit != new.suit {
            true => {
                if new.suit == tjall {
                    return new;
                }
                return highest;
            }
            false => {
                assert!(highest.value != new.value);
                if highest.value > new.value {
                    return highest;
                } else {
                    return new;
                }
            }
        }
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
        .await?;
    let game = game_repo
        .get_game_from_room(room.id)
        .await?;
    Ok(Json(game.clone()))
}
// create(start) game
#[post("/rooms/<room_id>/game")]
pub async fn create_game(
    room_id: usize,
    room_repo: &State<RoomRepository>,
    user_repo: &State<UserRepository>,
    game_repo: &State<GameRepository>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Game>, &'static str> {
    // first check if player is logged in before trying to create game; if not, no need to lock mutex
    let logged_in_user_id = LoginToken::try_refresh(cookies)?;

    let user = user_repo.get(logged_in_user_id).await?;
    if user.current_room != Some(room_id) {
        return Err("User is not in room!");
    }
    // get room
    let room = room_repo
        .get_room_by_id(room_id)
        .await?;
    let authorized = (user.role == Role::Admin) | (logged_in_user_id != room.host_id);
    if !authorized {
        return Err("Only the host can start the game!");
    }
    if room.players.len() < 3 {
        return Err("Not enough players in room to start game! You need at least 3 players.");
    }
    if let Ok(game) = game_repo.get_game_from_room(room.id).await {
        if game.is_in_progress() {
            return Err("Game already started!");
        }
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
    let user_id = LoginToken::try_refresh(cookies)?;
    let room = room_repo
        .get_room_by_id(room_id)
        .await?;
    let mut game = game_repo
        .get_game_from_room(room_id)
        .await?;
    let card = card.into_inner();
    let user = user_repo.get(user_id).await?;
    match game.game_state {
        GameState::Playing { ref current_round , tjall: _} => { //let player play card
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
            game.play_card(card, index)?;

        }
        GameState::Starting { ref remaining_deck } => { //let player pick tjall
            // check if user is starting player (or admin)
            let index = if let Role::Admin = user.role {
                // user is admin; allow them to play card as if they are current player
                Some(0)
            } else {
                // user is not admin; check if they are current player
                room.players.iter().position(|room_player| {
                    room_player.as_ref().map_or(false, |p| p.user_id == user_id)
                })
            }.ok_or("You are not a player in this room!")?;
            if index != 0 {
                return Err("You are not the starting player!");
            }
            let starting_player = &game.players[0];
            if !starting_player.current_cards.contains(&card){
                return Err("You do not have this card!");
            }
            // cloning remaining deck here to avoid borrowing as immutable on next line; no need to drain, because next game state does not have remaining deck
            let mut remaining_deck = remaining_deck.clone();
            game.pick_tjall_and_start(card.suit)?;
            let cards_amt = remaining_deck.len() / 4;
            for player in &mut game.players {
                player.current_cards.extend(remaining_deck.drain(0..cards_amt));
            }
        }
        _ => return Err("Game is not in progress!"),
    }
    Ok(Json(game.clone()))
    
}
#[delete("/rooms/<room_id>/game", format = "json")]
pub async fn forfeit(
    room_id: RoomId,
    game_repo: &State<GameRepository>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Game>, &'static str> {
    let user_id = LoginToken::try_refresh(cookies)?;
    let game = forfeit_player(game_repo, room_id, user_id).await?;
    Ok(Json(game))
}
pub async fn forfeit_player (game_repo: &State<GameRepository>, room_id: RoomId, user_id: UserId) -> Result<Game, &'static str>{
    let mut game = game_repo
        .get_game_from_room(room_id)
        .await?;
    // check if game is already finished
    if let GameState::Finished { .. } = game.game_state {
        return Err("Game is already finished!");
    }
    // determine winners based on if index is odd or even
    let room_player_index = game
        .players
        .iter()
        .position(|p| p.user_id == user_id)
        .ok_or("User is not a player in this game!")?;
    let winners = match room_player_index % 2 {
        0 => { //even
            [game.players[1].clone(), game.players[3].clone()]
        }
        1 => { // odd
            [game.players[0].clone(), game.players[2].clone()]
        }
        _ => unreachable!(),
    };
    //game is updated instead of actually deleted, because we need to remember last game's winners
    game.game_state = GameState::Finished {
        winners,
        reason: EndGameReason::Forfeit {
            player: game.players[room_player_index].clone(),
        },
    };
    game_repo.update_game(room_id, game.clone()).await;
    Ok(game)
}
#[get("/rooms/<room_id>/game/cards")]
pub async fn get_cards(
    room_id: RoomId,
    room_repo: &State<RoomRepository>,
    game_repo: &State<GameRepository>,
    user_repo: &State<UserRepository>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Vec<Card>>, &'static str> {
    let user_id = LoginToken::try_refresh(cookies)?;
    room_repo
        .get_room_by_id(room_id)
        .await?;
    let game = game_repo
        .get_game_from_room(room_id)
        .await?;
    user_repo.get(user_id).await?;

    let player = game
        .players
        .iter()
        .find(|p| p.user_id == user_id)
        .ok_or("You are not a player in this room!")?;
    if let GameState::Playing {
        current_round,
        tjall: _, } = game.game_state
    {
        let round_opening_card = &current_round.played_cards[0];
        let valid_cards = Game::valid_cards(&player, round_opening_card.suit);
        Ok(Json(valid_cards))
    } else {
        Err("Game is not in progress!")
    }
}

#[get("/rooms/<room_id>/game/cards/<player_index>")]
pub async fn get_cards_admin(
    room_id: RoomId,
    room_repo: &State<RoomRepository>,
    game_repo: &State<GameRepository>,
    user_repo: &State<UserRepository>,
    player_index: usize,
    cookies: &CookieJar<'_>,
) -> Result<Json<Vec<Card>>, &'static str> {
    let logged_in_user = user_repo.get(LoginToken::try_refresh(cookies)?).await?;
    if let Role::Admin = logged_in_user.role {
    } else {
        return Err("You are not an admin!");
    }

    // user is admin: get cards for player at index
    room_repo
        .get_room_by_id(room_id)
        .await?;
    let game = game_repo
        .get_game_from_room(room_id)
        .await?;

    let player = game.players[player_index].clone();
    if let GameState::Playing {
        current_round,
        tjall: _,
    } = game.game_state
    {
        let round_suit = current_round.played_cards[0].suit;
        let valid_cards = Game::valid_cards(&player, round_suit);
        Ok(Json(valid_cards))
    } else {
        Err("Game is not in progress!")
    }
}
// #[post("/rooms/<room_id>/game/tjall/<suit>")]
// pub async fn pick_tjall(
//     room_id: RoomId,
//     suit: Suit,
//     game_repo: &State<GameRepository>,
//     user_repo: &State<UserRepository>
// ) -> Result<Json<Game>, &'static str>{
//     // check if game is starting
//     let game = game_repo.get_game_from_room(room_id).await?;
//     unimplemented!();
//     // check if user is admin
//     // check if suit is valid
//     // pick tjall
//     // deal remaining cards
//     // 
//     // return game
// }