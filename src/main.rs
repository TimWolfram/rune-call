mod model;
mod repository;
use repository::*;

mod controller;
use controller::*;

use rocket::response::Redirect;

#[macro_use]
extern crate rocket;

#[launch]
/// Launches the rocket server
fn rocket() -> _ {
    rocket::build()
        //mount endpoints
        .mount(
            "/",
            routes![
                login::login,
                login::logout,
                login::register,
                
                rooms::get_rooms,
                rooms::get_room,
                rooms::create_room,

                games::get_game,
                games::create_game,
                // games::get_games, //games history - NYI

                secret,
            ],
        )
        //add state: using in-memory repositories instead of databases
        .manage(UserRepository::default())
        .manage(RoomRepository::default())
        .manage(GameRepository::default())
}

#[get("/secret")]
fn secret() -> Redirect {
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    Redirect::to(url) // ;)
}
