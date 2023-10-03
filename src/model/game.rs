use std::collections::HashMap;
use crate::model::{player::Player, rune::Rune, round::Round, card::Card, room::Room};

use rocket::{ response::{Responder, Response}, serde::json, Request, http::ContentType};
use serde::{Serialize, Deserialize};

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Game{
    pub id: usize,
    pub current_round: Round,
    pub player_order: Vec<Player>,
    pub played_rounds: Vec<Round>,
    pub player_cards: HashMap<Player, Vec<Card>>,
    pub blessed_rune: Option<Rune>, //"troef", always count as higher value than other cards
}

impl Game{
    pub fn new(id: usize, players: Vec<Player>) -> Game {
        Game {
            id: id,
            current_round: Round::new(players),
            blessed_rune: None,
            player_order: Vec::new(),
            played_rounds: Vec::new(),
            player_cards: HashMap::new(),
        }
    }

    pub fn start(room: &Room) -> Game {
        Game::new(room.id, room.current_players.clone())
    }
}

impl<'r> Responder<'r, 'static> for Game {
    fn respond_to(self, req: &Request) -> Result<Response<'static>, rocket::http::Status> {
        let json = json::to_string(&self).unwrap();
        Response::build()
            .header(ContentType::JSON)
            .sized_body(json.len(), std::io::Cursor::new(json))
            .ok()
    }
}