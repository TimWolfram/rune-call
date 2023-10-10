use std::hash::{Hash, Hasher};

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
       self.name == other.name
    }
}
impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl Eq for Player {}
