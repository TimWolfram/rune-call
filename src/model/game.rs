use std::collections::HashMap;
use crate::model::{player::Player, rune::Rune, round::Round, card::Card, room::Room};

use rocket::{ response::{Responder, Response}, serde::json, Request, http::ContentType};
use serde::{Serialize, Deserialize};

pub type PlayerInRoomListId = usize;
pub type TeamId = usize;
pub type PlayerTeams = HashMap<TeamId, Vec<PlayerInRoomListId>>;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Game{
    pub current_round: Round,
    pub player_order: Vec<Player>,
    pub teams: PlayerTeams,
    pub played_rounds: Vec<Round>,
    pub player_cards: HashMap<Player, Vec<Card>>,
    pub blessed_rune: Option<Rune>, //"troef", always count as higher value than other cards
}