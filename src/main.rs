mod model;
mod repository;
use repository::*;

mod controller;
use controller::*;

use rocket::{response::Redirect, http::Status};

#[macro_use]
extern crate rocket;

#[launch]
/// Launches the rocket server
fn rocket() -> _ {
    let build = rocket::build()
        //mount endpoints
        .mount(
            "/",
            routes![
                login::login,
                login::logout,
                login::register,
                login::delete_user,
                
                rooms::get_rooms_paged,
                rooms::get_rooms_public_paged,
                rooms::get_room,
                rooms::create_room,
                rooms::delete_room,

                rooms::join_room,
                rooms::leave_room,
                rooms::swap_player_seats,

                games::get_game,
                games::create_game,

                games::get_cards,
                games::get_cards_admin,
                games::play_card,

                games::forfeit,
                // games::get_games, //games history - NYI
                secret,
            ],
        )
        //add state: using in-memory repositories instead of databases
        .manage(UserRepository::default())
        .manage(RoomRepository::default())
        .manage(GameRepository::default());
    build
}

#[get("/secret", data="<form>")]
fn secret(form: Option<rocket::serde::json::Json<bool> >) -> Result<Redirect, (Status, &'static str)> {
    if let Some(form) = form {
        if form.into_inner() {
            return Ok(Redirect::to("https://www.youtube.com/watch?v=mh3L091Y7QQ"));
        }
        else {
            return Err((Status::Forbidden, "You are not worthy!"));
        }
    }
    Err((Status::ExpectationFailed, "What did you expect??"))
}