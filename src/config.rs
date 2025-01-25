use serde::{Deserialize, Deserializer, Serialize};
use std::{error::Error, fs::File, io::Read};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub containers: Vec<ContainerConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContainerConfig {
    /// The image to run, e.g. "example/cubbyman:latest"
    pub image: String,
    /// The name of the container, e.g. "cubbyman"
    pub name: String,
    /// The environment variables to set, e.g. ["USERNAME=admin", "PASSWORD=hunter2"]
    pub env: Option<Vec<String>>,
    /// The ports to bind, e.g. ["8602:8602"]
    #[serde(default, deserialize_with = "deserialize_ports")]
    pub ports: Option<Vec<Port>>,
    /// The IP address to bind the container to
    #[serde(default = "default_host_ip")]
    pub host_ip: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Port {
    pub host: i16,
    pub container: i16,
}

fn default_host_ip() -> String {
    "127.0.0.1".to_string()
}

fn deserialize_ports<'de, D>(deserializer: D) -> Result<Option<Vec<Port>>, D::Error>
where
    D: Deserializer<'de>,
{
    let ports: Option<Vec<String>> = Deserialize::deserialize(deserializer)?;
    match ports {
        Some(ports) => Ok(Some(
            ports
                .iter()
                .map(|p| Port {
                    host: p.split(':').next().unwrap().parse::<i16>().unwrap(),
                    container: p.split(':').last().unwrap().parse::<i16>().unwrap(),
                })
                .collect(),
        )),
        None => Ok(None),
    }
}

pub fn load_config_from_file(path: &str) -> Result<Config, Box<dyn Error>> {
    let mut buffer = String::new();
    let _ = File::open(path)?.read_to_string(&mut buffer)?;
    load_config_from_string(&buffer)
}
pub fn load_config_from_string(config_string: &str) -> Result<Config, Box<dyn Error>> {
    let config: Config = toml::from_str(config_string)?;
    Ok(config)
}
