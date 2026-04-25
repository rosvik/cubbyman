use crate::{traits::ToString, utils};
use clio::Input;
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default)]
    pub containers: Vec<ContainerConfig>,

    #[serde(default)]
    pub logins: Vec<Login>,

    #[serde(default)]
    include: Vec<PathBuf>,
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
    #[serde(default, deserialize_with = "deserialize_envs")]
    pub env: Vec<Env>,

    /// Secrets to load from .env file. Format is `dotenv_key:container_env_key`.
    #[serde(default, deserialize_with = "deserialize_secrets")]
    secrets: Option<Vec<Secret>>,

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

    /// The base directory to resolve paths relative to.
    #[serde(skip)]
    base_directory: Option<PathBuf>,
}

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
fn deserialize_envs<'de, D>(deserializer: D) -> Result<Vec<Env>, D::Error>
where
    D: Deserializer<'de>,
{
    let envs: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(envs.iter().map(|e| Env::from_string(e)).collect())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Secret {
    pub dotenv_key: String,
    pub container_env_key: String,
}

impl Secret {
    pub fn from_string(string: &str) -> Result<Self, std::io::ErrorKind> {
        let string = string.trim();
        if string.is_empty() {
            return Err(std::io::ErrorKind::InvalidInput);
        }
        let (dotenv_key, container_env_key) = string.split_once(':').unwrap_or((string, string));
        Ok(Self {
            dotenv_key: dotenv_key.to_string(),
            container_env_key: container_env_key.to_string(),
        })
    }
}

fn deserialize_secrets<'de, D>(deserializer: D) -> Result<Option<Vec<Secret>>, D::Error>
where
    D: Deserializer<'de>,
{
    let secrets: Option<Vec<String>> = Deserialize::deserialize(deserializer)?;
    match secrets {
        Some(secrets) => {
            let secrets: Vec<Secret> = secrets
                .iter()
                .map(|s| Secret::from_string(s))
                .filter(|s| s.is_ok())
                .flatten()
                .collect();
            Ok(Some(secrets))
        }
        None => Ok(None),
    }
}

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
fn deserialize_ports<'de, D>(deserializer: D) -> Result<Option<Vec<Port>>, D::Error>
where
    D: Deserializer<'de>,
{
    let ports: Option<Vec<String>> = Deserialize::deserialize(deserializer)?;
    Ok(ports.map(|ports| ports.iter().map(|p| Port::from_string(p)).collect()))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mount {
    /// The local path to bind to the container. Relative to the directory of the config file.
    pub host_path: PathBuf,
    /// The path to bind to the container.
    pub container_path: String,
}
impl Mount {
    pub fn from_string(string: &str) -> Self {
        let (host_path, container_path) = string.split_once(':').unwrap();
        Self {
            host_path: PathBuf::from(host_path),
            container_path: container_path.to_string(),
        }
    }
}
impl Display for Mount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host_path.to_string(), self.container_path)
    }
}
fn deserialize_mounts<'de, D>(deserializer: D) -> Result<Option<Vec<Mount>>, D::Error>
where
    D: Deserializer<'de>,
{
    let mounts: Option<Vec<String>> = Deserialize::deserialize(deserializer)?;
    Ok(mounts.map(|mounts| mounts.iter().map(|m| Mount::from_string(m)).collect()))
}

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
fn deserialize_volumes<'de, D>(deserializer: D) -> Result<Option<Vec<Volume>>, D::Error>
where
    D: Deserializer<'de>,
{
    let volumes: Option<Vec<String>> = Deserialize::deserialize(deserializer)?;
    Ok(volumes.map(|volumes| volumes.iter().map(|v| Volume::from_string(v)).collect()))
}

impl Config {
    pub fn from_str(config_str: &str) -> Result<Self, Box<dyn Error>> {
        let config: Config = toml::from_str(config_str)?;
        Ok(config)
    }

    /// Load a config file and recursively include other config files
    pub fn load(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let current_dir = std::env::current_dir().unwrap_or(PathBuf::from("."));
        let base_directory = path.parent().unwrap_or_else(|| &current_dir).to_path_buf();
        let mut config = Self::read(path)?;

        for container in config.containers.iter_mut() {
            // Store the config file's base directory for path resolution
            container.base_directory = Some(base_directory.clone());

            if let Some(secrets) = container.secrets.as_ref() {
                // Load the .env file and insert as environment variable
                let env_file = utils::load_env_in(base_directory.as_path())?;
                for secret in secrets.iter() {
                    if let Some(value) = env_file.get(secret.dotenv_key.as_str()) {
                        container.env.push(Env {
                            key: secret.container_env_key.clone(),
                            value: value.clone(),
                        });
                    };
                }
                // Clear the secrets list
                container.secrets = None;
            }
        }

        for include in config.include.iter() {
            // Recursively include with the new base directory
            let include_path = base_directory.join(include);
            let included_config = Self::load(&include_path)?;

            config.containers.extend(included_config.containers);
            config.logins.extend(included_config.logins);
        }

        // Clear the include list
        config.include = vec![];

        Ok(config)
    }

    fn read(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut buffer = String::new();
        let _ = File::open(path)?.read_to_string(&mut buffer)?;
        Self::from_str(&buffer)
    }
}

impl ContainerConfig {
    pub fn base_directory(&self) -> PathBuf {
        self.base_directory
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or(PathBuf::from(".")))
    }
}

pub fn path_or_default(input: Option<Input>) -> Option<PathBuf> {
    if let Some(input) = input {
        Some(input.path().to_path_buf())
    } else {
        get_default_config_path().map(PathBuf::from)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::ToRelative;

    #[tokio::test]
    async fn test_parse() {
        let config = include_str!("../tests/example.toml");
        let config = Config::from_str(config).unwrap();
        assert_eq!(config.containers.len(), 2);
        assert_eq!(config.containers[0].name, "container-cubby");
        assert_eq!(
            config.containers[0].image,
            "cubby.no/rosvik/container-cubby:main"
        );
        assert_eq!(config.containers[0].network, Some(String::from("cubby")));
        assert_eq!(config.containers[1].name, "hello");

        let secret = config.containers[0].secrets.as_ref().unwrap()[0].clone();
        assert_eq!(secret.dotenv_key, String::from("CONTAINER_CUBBY_PASSWORD"));
        assert_eq!(secret.container_env_key, String::from("PASSWORD"));
    }

    #[tokio::test]
    async fn test_include() {
        let config = include_str!("../tests/include1.toml");
        let config = Config::from_str(config).unwrap();
        assert_eq!(config.include.len(), 1);
        assert_eq!(config.containers.len(), 0);

        let config = Config::load(&PathBuf::from("tests/include1.toml")).unwrap();
        assert_eq!(config.include.len(), 0);
        assert_eq!(config.containers.len(), 3);
        let container_cubby = config
            .containers
            .iter()
            .find(|c| c.name == "container-cubby")
            .unwrap();
        assert_eq!(container_cubby.name, "container-cubby");
    }

    #[tokio::test]
    async fn test_load() {
        let config = Config::load(&PathBuf::from("tests/include1.toml")).unwrap();
        // Find env with key "PASSWORD"
        let container_cubby = config
            .containers
            .iter()
            .find(|c| c.name == "container-cubby")
            .unwrap();
        let password = container_cubby
            .env
            .iter()
            .find(|e| e.key == "PASSWORD")
            .unwrap();
        assert_eq!(password.value, String::from("hunter2"));

        // Find container with name "qr.248.no"
        let qr_248_no = config
            .containers
            .iter()
            .find(|c| c.name == "qr.248.no")
            .unwrap();
        let super_secret = qr_248_no
            .env
            .iter()
            .find(|e| e.key == "SUPER_SECRET")
            .unwrap();
        assert_eq!(super_secret.value, String::from("hello!"));

        // Find mount with name "test.txt"
        let test_txt = qr_248_no
            .mounts
            .as_ref()
            .unwrap()
            .iter()
            .find(|m| m.container_path == "test.txt")
            .unwrap();
        assert_eq!(
            test_txt.host_path.to_relative(&qr_248_no.base_directory()),
            String::from("tests/dir/test.txt")
        );
    }
}
