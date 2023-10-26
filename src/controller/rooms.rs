use crate::model::CreateRoomForm;
use crate::model::Player;
use crate::model::PlayerToken;
use crate::model::Room;
use crate::model::Rune;
use crate::repository::PlayerRepository;
use crate::repository::RoomRepository;

use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::State;

impl Room {
    pub fn add_player(&mut self, player: &Player, team: usize) -> bool {
        if self.player_teams.len() >= Rune::count() {
            return false;
        }
        self.player_teams.insert(team, vec![player.id]);
        true
    }
    pub fn switch_team(&mut self, player_id: usize, new_team: usize) -> bool {
        let player_teams = &mut self.player_teams;
        //find current player team
        for (team_id, players) in player_teams.iter_mut() {
            if players.contains(&player_id) {
                //remove player from current team
                players.retain(|p| *p != player_id);
                //add player to new team
                player_teams.entry(new_team).or_insert(Vec::new()).push(player_id);
                return true;
            }
        }
        false
    }
}


// return a list of all rooms
#[get("/rooms", format = "json")]
pub async fn get_rooms(room_repo: &State<RoomRepository>) -> Json<Vec<Room>> {
    Json(room_repo.rooms.lock().await.values().cloned().collect())
}

// get room by id (join room -- requires room code)
#[get("/rooms/<id>")]
pub async fn get_room(id: usize, room_repo: &State<RoomRepository>) -> Option<Json<Room>> {
    if let Some(room) = room_repo.rooms.lock().await.get(&id) {
        return Some(Json(room.clone()))
    }
    None
}

// create room
#[post("/rooms", format = "json", data = "<create_room_form>")]
pub async fn create_room<'a>(create_room_form: Json<CreateRoomForm>, 
                                room_repo: &State<RoomRepository>, 
                                player_repo: &State<PlayerRepository>,
                                cookies: &'a CookieJar<'a>) -> Result<Json<Room>, &'a str> 
{
    println!("create_room_form: {:?}", create_room_form);
    
    //get currently logged in player
    let player_token = PlayerToken::from_cookies(cookies);
    if let Err(e) = player_token {
        return Err(e)
    }
    let player_token = player_token.unwrap();
    //create room
    let rooms = room_repo.rooms.lock().await;
    if rooms.contains_key(&player_token.player_id) {
        return Json(Err("Player is already hosting a room!"));
    }

    Ok(
        Room::from_form(
            &host_player
            &create_room_form
        )
    )
}

// //delete room
#[delete("/rooms/<id>", format="json")]
pub async fn delete_room<'a>(id: usize, 
   room_repo: &'a State<RoomRepository>, player_repo: &'a State<RoomRepository>, cookies: &'a CookieJar<'a>) -> Result<(), &'a str> {
    //get currently logged in player
    let jwt = cookies.get_private("token");
    if jwt.is_none(){
        return Err("Cannot delete room: user is not logged in!");
    }
    let jwt = jwt.unwrap().value();
    let claims = PlayerToken::decode(&jwt);
    
    // let rooms = room_repo.rooms.lock().await;
    
    // let host_player_opt = player_repo.get_player_by_id(player.value());
    // Json(rooms.remove(&id).is_some())
    Ok(())
   }

//join room
// #[put("/rooms/<room_id>/players", format = "json")]
// pub fn put_room(room_id: usize, cookies: &rocket::http::CookieJar) -> Json<Option<Room>> {
//     cookies.get("token");

//     // Decode JWT token from cookie
//     let token_cookie = cookies.get_private("token");
//     // Decode JWT token
//     let token_message = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256));

//     let token = jsonwebtoken::decode::<jwt::PlayerToken>(x, &DecodingKey::from_secret(x), ).unwrap();
// }

