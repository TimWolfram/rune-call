mod model;

mod repository;
use repository::*;

mod endpoints;

// use endpoints::*;
mod test;

#[macro_use] extern crate rocket;

#[launch]
/// Launches the rocket server
fn rocket() -> _ {
    rocket::build()
    //mount endpoints
    // .mount("/", routes![
    //         games::get_games,
    //         games::get_game,
    //         games::create_game,
    //         rooms::get_rooms,
    //     ]
    // )
    .mount("/test/", routes![
            test::id
        ]
    )
    
    //add state
    .manage(PlayerRepository::default())
    .manage(RoomRepository::default())
}