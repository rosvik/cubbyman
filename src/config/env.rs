use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Env {
    pub key: String,
    pub value: String,
}
impl Env {
    pub fn from_string(string: &str) -> Self {
        let (key, value) = string.split_once('=').unwrap();
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}
impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}
pub fn deserialize_envs<'de, D>(deserializer: D) -> Result<Vec<Env>, D::Error>
where
    D: Deserializer<'de>,
{
    let envs: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(envs.iter().map(|e| Env::from_string(e)).collect())
}
