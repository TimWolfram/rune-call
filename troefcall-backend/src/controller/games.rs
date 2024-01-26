use crate::model::game::{Card, EndGameReason, Game, GameState, Player, Room, RoomId, Round, RoundState, Suit};
use crate::model::login::{LoginToken, Role, UserId};
use crate::repository::{GameRepository, RoomRepository, UserRepository};

use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket::State;

type Error<'a> = (Status, &'a str);

impl Game {
    /// Get the order of players for a new game.
    fn player_order(room: &Room, last_game: Option<Game>) -> Result<[Player; 4], Error<'static>> {
        // check if room has enough players
        let players = room.players.clone();
        for player in &players {
            if let None = player {
                return Err((Status::BadRequest, "Room does not have enough players!"));
            }
        }

        // convert players to array
        let mut players: [Player; 4] = players
            .into_iter()
            .map(|player| player.unwrap())
            .collect::<Vec<Player>>()
            .try_into()
            .or(Err((
                Status::Conflict,
                "Cannot start the game: room should have 4 players!",
            )))?;

        // check who the next starting player should be based on who won last game
        enum StartingPlayer {
            Last,
            Next,
            Random,
        }
        let starting_player = match last_game {
            None => StartingPlayer::Random, //pick random starting player
            Some(game) => {
                let GameState::Finished { winners, reason: _ } = game.state else {
                    return Err((Status::BadRequest, "Last game is not finished!"));
                };
                // check if starting player won
                let last_starting = &game.players[0];
                match winners.contains(last_starting) {
                    true => StartingPlayer::Last,
                    false => StartingPlayer::Next,
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
    pub fn create(room: &mut Room, last_game: Option<Game>) -> Result<Game, Error<'static>> {
        let mut players = Game::player_order(room, last_game)?;
        //draw 5 cards for each player
        let mut deck: Vec<Card> = Card::generate_deck();
        for player in &mut players {
            player.current_cards = deck.drain(0..5).collect();
        }
        let game = Game {
            players,
            played_rounds: Vec::new(),
            state: GameState::Starting {
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
        match self.state {
            GameState::Finished { .. } => false,
            _ => true,
        }
    }

    /// Pick a tjall suit aka trump aka "troef".
    /// This starts the game; changing its state from `Starting` to `Playing`.
    pub fn pick_tjall_and_start(&mut self, suit: Suit) -> Result<(), Error<'static>> {
        let GameState::Starting { remaining_deck: _ } = self.state else {
            return Err((Status::Conflict, "Game is not starting!"));
        };

        self.state = GameState::Playing {
            current_round: Round::new(0),
            tjall: suit,
        };
        Ok(())
    }

    /// Play a card for the current player.
    pub fn play_card(&mut self, card: Card, index: usize) -> Result<(), Error<'static>> {
        // check if player has card
        let player = &self.players[index];
        if !player.current_cards.contains(&card) {
            println!("Player {} does not have card {}", index, card.to_string());
            return Err((Status::BadRequest, "You do not have this card!"));
        } 
        let GameState::Playing {
            ref mut current_round,
            tjall,
        } = self.state
        else {
            return Err((Status::Conflict, "Game is not in progress!"));
        };
        if current_round.played_cards.len() > 0 {
            let round_opening_card = &current_round.played_cards[0];
            let valid_cards = Game::valid_cards(player, round_opening_card.suit);
            if !valid_cards.contains(&card) {
                return Err(
                    (Status::Conflict,
                    "You cannot play this card! If you have a card of the same suit, you must play it.")
                );
            }
        }
        current_round.played_cards.push(card.clone());
        // remove card from player's hand
        self.players[index]
            .current_cards
            .retain(|c| c.clone() != card);
        // check if round is finished
        if current_round.played_cards.len() == 4 {
            // determine winner
            let mut winning_card = &current_round.played_cards[0];
            for played_card in &current_round.played_cards {
                winning_card = Card::compare(winning_card, played_card, tjall);
            }
            let winning_card_index = current_round
                .played_cards
                .iter()
                .position(|c| c == winning_card)
                .unwrap();
            current_round.set_winner(winning_card_index);
            // add round to played rounds
            self.played_rounds.push(current_round.clone());
            // check if game is finished
            // count scores
            let mut team_1_score = 0;
            let mut team_2_score = 0;
            for round in &self.played_rounds {
                if let RoundState::RoundWon { winner_user_id: winner_index } = round.state {
                    match (winner_index + round.player_starting) % 2 {
                        0 => team_1_score += 1,
                        1 => team_2_score += 1,
                        _ => unreachable!(), //should never happen
                    }
                }
            }
            if team_1_score >= 7 {
                // team 1 wins
                let winners = [self.players[0].clone(), self.players[2].clone()];
                self.state = GameState::Finished {
                    winners,
                    reason: EndGameReason::Score {
                        score: [team_1_score, team_2_score],
                    },
                };
            } else if team_2_score >= 7 {
                // team 2 wins
                let winners = [self.players[1].clone(), self.players[3].clone()];
                self.state = GameState::Finished {
                    winners,
                    reason: EndGameReason::Score {
                        score: [team_1_score, team_2_score],
                    },
                };
            } else { // game is not finished; start new round
                //check who won this round
                let winner_index = match current_round.state {
                    RoundState::RoundWon { winner_user_id } => winner_user_id,
                    _ => unreachable!(), //should never happen
                };
                self.state = GameState::Playing {
                    current_round: Round::new(winning_card_index),
                    tjall,
                };
            }
            
        }
        Ok(())
    }
}

impl Card {
    pub fn compare<'a>(highest: &'a Card, new: &'a Card, tjall: Suit) -> &'a Card {
        match highest.suit == new.suit {
            // if suit is different, new wins if suit is tjall
            false if new.suit == tjall => new,
            false => highest,

            // if suit is same, new wins if value is higher
            true if highest.value > new.value => highest,
            true => new,
        }
    }
}

//get game by id (join game -- requires room code)
#[get("/<room_id>/game")]
pub async fn get_game<'a>(
    room_id: usize,
    room_repo: &'a State<RoomRepository>,
    game_repo: &'a State<GameRepository>,
) -> Result<Json<Game>, Error<'a>> {
    let room = room_repo.get_room_by_id(room_id).await?;
    let game = game_repo.get_game_from_room(room.id).await?;
    Ok(Json(game.clone()))
}

// create(start) game
#[post("/<room_id>/game")]
pub async fn create_game<'a>(
    room_id: usize,
    room_repo: &'a State<RoomRepository>,
    user_repo: &'a State<UserRepository>,
    game_repo: &'a State<GameRepository>,
    cookies: &'a CookieJar<'a>,
) -> Result<Json<Game>, Error<'a>> {
    // first check if player is logged in before trying to create game; if not, no need to lock mutex
    let logged_in_user_id = LoginToken::refresh_jwt(cookies)?;

    let user = user_repo.get(logged_in_user_id).await?;
    if user.current_room != Some(room_id) {
        return Err((Status::Unauthorized, "User is not in room!"));
    }
    // get room
    let mut room = room_repo.get_room_by_id(room_id).await?;

    let authorized = (user.role == Role::Admin) | (logged_in_user_id != room.host_id);
    if !authorized {
        return Err((Status::Unauthorized, "Only the host can start the game!"));
    }
    if room.players.len() < 3 {
        return Err((
            Status::BadRequest,
            "Not enough players in room to start game! You need at least 3 players.",
        ));
    }
    if let Ok(game) = game_repo.get_game_from_room(room.id).await {
        if game.is_in_progress() {
            return Err((Status::BadRequest, "Game already started!"));
        }
    }
    room.game_in_progress = true;
    let game = game_repo.create_game(room.clone()).await?;
    room_repo.update_room(room).await;

    Ok(Json(game.clone()))
}

#[put("/<room_id>/game", data = "<card>", format = "json")]
pub async fn play_card<'a>(
    room_id: RoomId,
    room_repo: &'a State<RoomRepository>,
    game_repo: &'a State<GameRepository>,
    user_repo: &'a State<UserRepository>,
    card: Json<Card>,
    cookies: &'a CookieJar<'a>,
) -> Result<Json<Game>, Error<'a>> {
    let user_id = LoginToken::refresh_jwt(cookies)?;
    let room = room_repo.get_room_by_id(room_id).await?;
    let mut game = game_repo.get_game_from_room(room_id).await?;

    let card = card.into_inner();
    let user = user_repo.get(user_id).await?;

    match game.state {
        GameState::Playing { ref current_round, tjall: _, } => 
        {
            // check if it is player's turn (or player is admin)
            let player_turn = (current_round.played_cards.len() + current_round.player_starting) % 4;
            let current_player_index = match user.role {
                Role::Admin => {
                    // user is admin; allow them to play card as if they are current player
                    Some(player_turn)
                }
                _ => {
                    // user is not admin; get id of current player
                    game.players.iter().position(|game_player| {
                        game_player.user_id == user_id
                    })
                }
            }.ok_or((Status::Unauthorized, "You are not a player in this room!"))? + current_round.player_starting;

            if current_player_index != player_turn {
                println!("Player {} is not the current player! Player turn: {}\nPlayer order: {}", current_player_index, player_turn, game.players.iter().map(|p| p.name.clone()).collect::<Vec<String>>().join(", "));
                return Err((Status::BadRequest, "It is not your turn!"));
            }
            game.play_card(card, current_player_index)?;
            game_repo.update_game(room_id, game.clone()).await;
        }
        GameState::Starting { ref remaining_deck } => 
        {
            //let player pick tjall
            // check if user is starting player (or admin)
            let index = if let Role::Admin = user.role {
                // user is admin; allow them to play card as if they are current player
                Some(0)
            } else {
                // user is not admin; check if they are current player
                game.players.iter().position(|game_player| {
                    game_player.user_id == user_id
                })
            }
            .ok_or((Status::Unauthorized, "You are not a player in this room!"))?;

            if index != 0 {
                println!("Player {} is not the starting player!\nPlayer order: {}", index, game.players.iter().map(|p| p.name.clone()).collect::<Vec<String>>().join(", "));
                return Err((Status::Unauthorized, "You are not the starting player!"));
            }
            let starting_player = &game.players[0];
            if !starting_player.current_cards.contains(&card) {
                println!("Player {} does not have card {}", starting_player.name, card.to_string());
                return Err((Status::BadRequest, "You do not have this card!"));
            }
            // cloning remaining deck here to avoid borrowing as immutable on next line; no need to drain, because next game state does not have remaining deck
            let mut remaining_deck = remaining_deck.clone();
            game.pick_tjall_and_start(card.suit)?;
            let cards_amt = remaining_deck.len() / 4;
            for player in &mut game.players {
                player
                .current_cards
                .extend(remaining_deck.drain(0..cards_amt));
            }
            game_repo.update_game(room_id, game.clone()).await;
        }
        _ => return Err((Status::Conflict, "Game is not in progress!")),
    }
    Ok(Json(game))
}

#[delete("/<room_id>/game")]
pub async fn forfeit<'a>(
    room_id: RoomId,
    game_repo: &'a State<GameRepository>,
    room_repo: &'a State<RoomRepository>,
    cookies: &'a CookieJar<'_>,
) -> Result<Json<Game>, Error<'a>> {
    let user_id = LoginToken::refresh_jwt(cookies)?;
    let game = forfeit_player(game_repo, room_repo, room_id, user_id).await?;
    Ok(Json(game))
}
pub async fn forfeit_player<'a>(
    game_repo: &'a State<GameRepository>,
    room_repo: &'a State<RoomRepository>,
    room_id: RoomId,
    user_id: UserId,
) -> Result<Game, Error<'a>> {
    let mut game = game_repo.get_game_from_room(room_id).await?;
    // check if game is already finished
    if let GameState::Finished { .. } = game.state {
        // set game in progress to false on room
        room_repo.end_game(room_id).await;
        return Err((Status::Conflict, "Game is already finished!"));
    }
    // determine winners based on if index is odd or even
    let room_player_index = game
        .players
        .iter()
        .position(|p| p.user_id == user_id)
        .ok_or((Status::Unauthorized, "User is not a player in this game!"))?;
    
    // set game in progress to false on room
    room_repo.end_game(room_id).await;
    
    let winners = match room_player_index % 2 {
        0 => {
            //even
            [game.players[1].clone(), game.players[3].clone()]
        }
        1 => {
            // odd
            [game.players[0].clone(), game.players[2].clone()]
        }
        _ => unreachable!(), //should never happen
    };

    //game is updated instead of actually deleted, because we need to remember last game's winners when starting a new game
    game.state = GameState::Finished {
        winners,
        reason: EndGameReason::Forfeit {
            player: game.players[room_player_index].clone(),
        },
    };
    
    game_repo.update_game(room_id, game.clone()).await;
    Ok(game)
}

#[get("/<room_id>/game/cards")]
pub async fn get_cards<'a>(
    room_id: RoomId,
    room_repo: &'a State<RoomRepository>,
    game_repo: &'a State<GameRepository>,
    user_repo: &'a State<UserRepository>,
    cookies: &'a CookieJar<'a>,
) -> Result<Json<Vec<Card>>, Error<'a>> {
    let user_id = LoginToken::refresh_jwt(cookies)?;
    room_repo.get_room_by_id(room_id).await?;
    let game = game_repo.get_game_from_room(room_id).await?;
    user_repo.get(user_id).await?;

    let player = game
        .players
        .iter()
        .find(|p| p.user_id == user_id)
        .ok_or((Status::Unauthorized, "You are not a player in this room!"))?;
    return Ok(Json(player.current_cards.clone()));
}

#[get("/<room_id>/game/cards/<player_index>")]
pub async fn get_cards_admin<'a>(
    room_id: RoomId,
    room_repo: &'a State<RoomRepository>,
    game_repo: &'a State<GameRepository>,
    user_repo: &'a State<UserRepository>,
    player_index: usize,
    cookies: &CookieJar<'_>,
) -> Result<Json<Vec<Card>>, Error<'a>> {
    let logged_in_user = user_repo.get(LoginToken::refresh_jwt(cookies)?).await?;
    if logged_in_user.role != Role::Admin {
        return Err((Status::Unauthorized, "You are not an admin!"));
    };

    // user is admin: get cards for player at index
    room_repo.get_room_by_id(room_id).await?;
    let game = game_repo.get_game_from_room(room_id).await?;

    let player = game.players[player_index].clone();
    let GameState::Playing {
        current_round,
        tjall: _,
    } = game.state
    else {
        return Err((Status::Unauthorized, "Game is not in progress!"));
    };
    let round_suit = current_round.played_cards[0].suit;
    let valid_cards = Game::valid_cards(&player, round_suit);
    Ok(Json(valid_cards))
}
