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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContainerConfig {
    /// The image to run, e.g. "example/cubbyman:latest"
    pub image: String,

    /// The name of the container, e.g. "cubbyman"
    pub name: String,

    /// Environment variables on the format `"KEY=value"`.
    pub env: Option<Vec<String>>,

    /// The ports to bind, e.g. ["8602:8602"] (host:container)
    #[serde(default, deserialize_with = "deserialize_ports")]
    pub ports: Option<Vec<Port>>,

    /// The IP address to bind the container to (default: 127.0.0.1)
    #[serde(default = "default_host_ip")]
    pub host_ip: String,

    /// The local directories to bind to the container. Format is
    /// `host_path:container_path`.
    #[serde(default, deserialize_with = "deserialize_mounts")]
    pub mounts: Option<Vec<Mount>>,
}

fn default_host_ip() -> String {
    "127.0.0.1".to_string()
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
                    container: p.split(':').last().unwrap().parse::<i16>().unwrap(),
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
                container_path: m.split(':').last().unwrap().to_string(),
            })
            .collect()
    }))
}
fn to_absolute_path(path: &str) -> String {
    let mut path = PathBuf::from(path);
    if !path.is_absolute() {
        path = std::env::current_dir().unwrap().join(path);
        path = path.canonicalize().unwrap();
    }
    path.to_string_lossy().to_string()
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

/// Will look for a `cubbyfile.toml` in the following locations, in this
/// prioritized order:
/// 1. The user provided path
/// 2. `cubbyfile.toml` in the current working directory
/// 3. `.cubbyfile.toml` in the user's home directory
fn get_default_config_path() -> Option<String> {
    let path = Path::new("cubbyfile.toml");
    if path.exists() {
        println!(
            "Using cubbyfile.toml in current directory: {}",
            path.to_string_lossy()
        );
        return Some(path.to_string_lossy().to_string());
    }

    let mut path = PathBuf::from(std::env::var("HOME").unwrap());
    path.push(".cubbyfile.toml");
    if path.exists() {
        println!(
            "Using .cubbyfile.toml in home directory: {}",
            path.to_string_lossy()
        );
        return Some(path.to_string_lossy().to_string());
    }

    println!("No configuration file found");
    None
}
