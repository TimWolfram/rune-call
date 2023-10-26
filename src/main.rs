mod repository;
use repository::*;

mod model;

mod controller;
use controller::*;

mod password;
mod test;

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
                rooms::get_rooms,
                rooms::get_room,
                rooms::create_room,

                games::get_game,
                games::create_game,
                // games::get_games, //games history - NYI

                secret,
            ],
        )
        .mount("/test/", routes![test::id])
        //add state
        .manage(PlayerRepository::default())
        .manage(RoomRepository::default())
}

#[get("/secret")]
fn secret() -> Redirect {
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    Redirect::to(url) // ;)
}
