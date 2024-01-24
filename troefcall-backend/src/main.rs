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

const CREATE_TEST_DATA: bool = true;

#[launch]
/// Launches the rocket server
async fn rocket() -> _ {
    //create repositories
    let mut user_repo = match CREATE_TEST_DATA {
            true => UserRepository::test_repo(),
            false => UserRepository::default(),
        };
    let room_repo = 
        if CREATE_TEST_DATA{RoomRepository::test_repo(&mut user_repo)}
        else {RoomRepository::default()};
    //print users
    println!("Users: {}", rocket::serde::json::to_string(&user_repo.users.get_mut()).unwrap());

    rocket::build()
        //mount endpoints
        .mount("/login", routes![
            login::login, 
            login::logout, 
            login::register, 
            login::change_nickname,
            login::delete_user])
        .mount("/rooms", routes![
            //rooms
            rooms::get_rooms_paged,
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
        .manage(user_repo)
        .manage(room_repo)
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