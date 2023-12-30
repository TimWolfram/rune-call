mod model;
mod repository;
use repository::*;

mod controller;
use controller::*;

mod cors;

mod test;

use rocket::{response::Redirect, http::Status};

#[macro_use]
extern crate rocket;

#[launch]
/// Launches the rocket server
fn rocket() -> _ {
    rocket::build()
        //mount endpoints
        .mount("/login", routes![
            login::testadmin, 
            login::login, 
            login::logout, 
            login::register, 
            login::delete_user])
        .mount("/rooms", routes![
            //rooms
            rooms::get_rooms_paged,
            rooms::get_rooms_public_paged,

            rooms::get_room,
            rooms::create_room,
            rooms::delete_room,
            rooms::join_room,
            rooms::leave_room,
            rooms::swap_player_seats,

            //games
            games::get_game,
            games::create_game,

            games::get_cards,
            games::get_cards_admin,
            games::play_card,

            games::forfeit,
        ])
        .mount("/", routes![
            secret,
        ])
        //add state: using in-memory repositories instead of databases
        .manage(UserRepository::test_repo())
        .manage(RoomRepository::test_repo())
        .manage(GameRepository::default())
        // add cors fairing
        .attach(cors::CORS)
}

type ErrorType = (Status, &'static str);

#[get("/secret", data="<form>")]
fn secret(form: Option<rocket::serde::json::Json<bool> >) -> Result<Redirect, ErrorType> {
    let Some(f) = form else {
        return Err((Status::ExpectationFailed, "What did you expect??"));
    };
    if f.into_inner() {
        return Ok(Redirect::to("https://www.youtube.com/watch?v=mh3L091Y7QQ"));
    }
    else {
        return Err((Status::Forbidden, "You are not worthy!"));
    }
}