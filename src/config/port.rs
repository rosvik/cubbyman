use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Port {
    pub host: i16,
    pub container: i16,
}
impl Port {
    pub fn from_string(string: &str) -> Self {
        let (host, container) = string.split_once(':').unwrap();
        Self {
            host: host.parse::<i16>().unwrap(),
            container: container.parse::<i16>().unwrap(),
        }
    }
}
impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.container)
    }
}
pub fn deserialize_ports<'de, D>(deserializer: D) -> Result<Option<Vec<Port>>, D::Error>
where
    D: Deserializer<'de>,
{
    let ports: Option<Vec<String>> = Deserialize::deserialize(deserializer)?;
    Ok(ports.map(|ports| ports.iter().map(|p| Port::from_string(p)).collect()))
}
