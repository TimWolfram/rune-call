use crate::model::CreateRoomForm;
use crate::model::Room;
use crate::repository::PlayerRepository;
use crate::repository::RoomRepository;

use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::State;

#[get("/rooms")]
pub async fn get_rooms(room_repo: &State<RoomRepository>) -> Json<Vec<Room>> {
    Json( room_repo.rooms.lock().await.values().cloned().collect() )
}

//get room by id (join room -- requires room code)
#[get("/rooms/<id>")]
pub async fn get_room(id: usize, room_repo: &State<RoomRepository>) -> Json<Option<Room>> {
    Json(room_repo.rooms.lock().await.get(&id).cloned())
}

//create room
#[post("/rooms", format = "json", data = "<create_room_form>")]
pub async fn create_room(create_room_form: Json<CreateRoomForm>, 
                        room_repo: &State<RoomRepository>, 
                        player_repo: &State<PlayerRepository>) 
                        -> Json<Option<Room>> {
    println!("create_room_form: {:?}", create_room_form);

    let host_player_opt = player_repo.players.get(&(create_room_form.host_id as usize));
    if host_player_opt.is_none() {
        return Json(None);
    }
    let host_player = host_player_opt.unwrap();
    let room_name = &create_room_form.name;
    let room_pwd = &create_room_form.password;
    let mut rooms = room_repo.rooms.lock().await;
    let id = rooms.len();
    let room = Room::create(id, host_player, room_name, room_pwd);
    rooms.insert(id, Room::create(id, host_player, room_name, room_pwd));
    let json = Json(Some(room));
    println!("json: {:?}", json);
    json
}

// //delete room
#[delete("/rooms/<id>")]
pub async fn delete_room(id: usize, room_repo: &State<RoomRepository>) -> Json<bool> {
    Json(room_repo.rooms.lock().await.remove(&id).is_some())
}

// //join room
// #[put("/rooms/<room_id>/players", format = "json", data = "<player>")]
// pub fn put_room(room_id: usize, player: PlayerToken) -> Json<Option<Room>> {
//     todo!();
// }
