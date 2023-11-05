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

#[get("/secret")]
fn secret() -> Redirect {
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    Redirect::to(url) // ;)
}
