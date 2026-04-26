use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Login {
    pub registry: String,
    pub username: String,
    pub password: String,
}
impl Hash for Login {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.registry.hash(state);
        self.username.hash(state);
    }
}
impl PartialEq for Login {
    fn eq(&self, other: &Self) -> bool {
        self.registry == other.registry && self.username == other.username
    }
}
impl Eq for Login {}
