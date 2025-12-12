use clio::Input;
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    error::Error,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub containers: Vec<ContainerConfig>,
    pub logins: Option<Vec<Login>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Login {
    pub registry: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContainerConfig {
    /// The image to run, e.g. "example/cubbyman:latest"
    pub image: String,

    /// The name of the container, e.g. "cubbyman"
    pub name: String,

    /// Arguments passed to the container on startup
    pub cmd: Option<Vec<String>>,

    /// Environment variables on the format `"KEY=value"`.
    pub env: Option<Vec<String>>,

    /// The ports to bind, e.g. ["8602:8602"] (host:container)
    #[serde(default, deserialize_with = "deserialize_ports")]
    pub ports: Option<Vec<Port>>,

    /// Network name or mode. Modes are `bridge`, `host`,
    /// `none`, and `container:<name|id>`. Any other value
    /// will be used to set up a custom bridge network.
    pub network: Option<String>,

    /// The local directories to bind to the container. Format is
    /// `host_path:container_path`.
    #[serde(default, deserialize_with = "deserialize_mounts")]
    pub mounts: Option<Vec<Mount>>,

    /// The volumes to bind to the container. Format is
    /// `volume_name:container_path`.
    #[serde(default, deserialize_with = "deserialize_volumes")]
    pub volumes: Option<Vec<Volume>>,

    /// The user to run the container as. Format is `user:group`.
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Port {
    pub host: i16,
    pub container: i16,
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
                    container: p.split(':').next_back().unwrap().parse::<i16>().unwrap(),
                })
                .collect(),
        )),
        None => Ok(None),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mount {
    pub host_path: String,
    pub container_path: String,
}
fn deserialize_mounts<'de, D>(deserializer: D) -> Result<Option<Vec<Mount>>, D::Error>
where
    D: Deserializer<'de>,
{
    let mounts: Option<Vec<String>> = Deserialize::deserialize(deserializer)?;
    Ok(mounts.map(|mounts| {
        mounts
            .iter()
            .map(|m| Mount {
                host_path: to_absolute_path(m.split(':').next().unwrap()),
                container_path: m.split(':').next_back().unwrap().to_string(),
            })
            .collect()
    }))
}
fn to_absolute_path(path: &str) -> String {
    let mut path = PathBuf::from(path);
    if !path.is_absolute() {
        path = std::env::current_dir().unwrap().join(path);
        path = path.canonicalize().unwrap_or_else(|e| {
            panic!("Failed to resolve path '{}': {}", path.to_string_lossy(), e)
        });
    }
    path.to_string_lossy().to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Volume {
    pub name: String,
    pub container_path: String,
}
fn deserialize_volumes<'de, D>(deserializer: D) -> Result<Option<Vec<Volume>>, D::Error>
where
    D: Deserializer<'de>,
{
    let volumes: Option<Vec<String>> = Deserialize::deserialize(deserializer)?;
    Ok(volumes.map(|volumes| {
        volumes
            .iter()
            .map(|v| Volume {
                name: v.split(':').next().unwrap().to_string(),
                container_path: v.split(':').next_back().unwrap().to_string(),
            })
            .collect()
    }))
}

pub fn load_config(cli_input: Option<Input>) -> Result<Config, Box<dyn Error>> {
    let mut buffer = String::new();

    if let Some(mut file) = cli_input {
        let _ = file.read_to_string(&mut buffer)?;
    } else if let Some(path) = get_default_config_path() {
        let mut file = File::open(path)?;
        let _ = file.read_to_string(&mut buffer)?;
    }
    load_config_from_string(&buffer)
}
fn load_config_from_string(config_string: &str) -> Result<Config, Box<dyn Error>> {
    let config: Config = toml::from_str(config_string)?;
    Ok(config)
}

/// Will look for a `cubbyfile.toml` in the current directory
fn get_default_config_path() -> Option<String> {
    let path = Path::new("cubbyfile.toml");
    if path.exists() {
        println!(
            "Using cubbyfile.toml in current directory: {}",
            path.to_string_lossy()
        );
        return Some(path.to_string_lossy().to_string());
    }

    println!("No configuration file found");
    None
}
