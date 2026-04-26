use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Volume {
    pub name: String,
    pub container_path: String,
}
impl Volume {
    pub fn from_string(string: &str) -> Self {
        let (name, container_path) = string.split_once(':').unwrap();
        Self {
            name: name.to_string(),
            container_path: container_path.to_string(),
        }
    }
}
impl Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.container_path)
    }
}
pub fn deserialize_volumes<'de, D>(deserializer: D) -> Result<Option<Vec<Volume>>, D::Error>
where
    D: Deserializer<'de>,
{
    let volumes: Option<Vec<String>> = Deserialize::deserialize(deserializer)?;
    Ok(volumes.map(|volumes| volumes.iter().map(|v| Volume::from_string(v)).collect()))
}
