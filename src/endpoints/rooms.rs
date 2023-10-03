use crate::model::CreateRoomForm;
use crate::model::Player;
use crate::model::Room;
use crate::repository::PlayerRepository;
use crate::repository::RoomRepository;

use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::State;

#[get("/rooms")]
pub fn get_rooms(room_repo: &State<RoomRepository>) -> Json<Vec<Room>> {
    Json(room_repo.rooms)
}

//get room by id (join room -- requires room code)
#[get("/rooms/<id>")]
pub fn get_room(id: usize, room_repo: &State<RoomRepository>) -> Json<Option<Room>> {
    let room = room_repo.rooms[id];
    Json(Some(room))
}

//create room
#[post("/rooms", format = "json", data = "<create_room_form>")]
pub fn create_room(create_room_form: Form<CreateRoomForm>, room_repo: &State<RoomRepository>, player_repo: &State<PlayerRepository>) 
                    -> Option<Json<Room>> {
    let host_player = player_repo.players.get(create_room_form.host_player_id)?;
    let room_name = &create_room_form.name;
    let room_pwd = &create_room_form.password;
    let &created_room = room_repo.create_room(host_player, room_name, room_pwd);
    
    Some(Json(created_room))
}

// //join room
// #[put("/rooms/<room_id>/players", format = "json", data = "<player>")]
// pub fn put_room(room_id: usize, player: Player) -> Json<Option<Room>> {
//     todo!();
// }
