use std::collections::HashMap;
use usize as PlayerId;
use u8 as TeamId;
type TeamPlayerList= Vec<PlayerId>; 

pub struct CreateGameForm {
    team_players: HashMap<TeamId, TeamPlayerList>
}