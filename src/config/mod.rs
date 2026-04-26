use crate::{
    config::{env::Env, login::Login, mount::Mount, port::Port, secret::Secret, volume::Volume},
    utils,
};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::Read, path::PathBuf};
mod env;
mod login;
mod mount;
mod port;
mod secret;
mod volume;

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
pub struct ContainerConfig {
    /// The image to run, e.g. "example/cubbyman:latest"
    pub image: String,

    /// The name of the container, e.g. "cubbyman"
    pub name: String,

    /// Arguments passed to the container on startup
    pub cmd: Option<Vec<String>>,

    /// Environment variables on the format `"KEY=value"`.
    #[serde(default, deserialize_with = "env::deserialize_envs")]
    pub env: Vec<Env>,

    /// Secrets to load from .env file. Format is `dotenv_key:container_env_key`.
    #[serde(default, deserialize_with = "secret::deserialize_secrets")]
    secrets: Option<Vec<Secret>>,

    /// The ports to bind, e.g. ["8602:8602"] (host:container)
    #[serde(default, deserialize_with = "port::deserialize_ports")]
    pub ports: Option<Vec<Port>>,

    /// Network name or mode. Modes are `bridge`, `host`,
    /// `none`, and `container:<name|id>`. Any other value
    /// will be used to set up a custom bridge network.
    pub network: Option<String>,

    /// The local directories to bind to the container. Format is
    /// `host_path:container_path`.
    #[serde(default, deserialize_with = "mount::deserialize_mounts")]
    pub mounts: Option<Vec<Mount>>,

    /// The volumes to bind to the container. Format is
    /// `volume_name:container_path`.
    #[serde(default, deserialize_with = "volume::deserialize_volumes")]
    pub volumes: Option<Vec<Volume>>,

    /// The user to run the container as. Format is `user:group`.
    pub user: Option<String>,

    /// The base directory to resolve paths relative to.
    #[serde(skip)]
    pub base_directory: Option<PathBuf>,
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

        config.check()
    }

    fn read(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut buffer = String::new();
        let _ = File::open(path)?.read_to_string(&mut buffer)?;
        Self::from_str(&buffer)
    }

    /// Checks the config for errors and warnings.
    /// - Removes duplicate logins
    /// - Throws an error if several containers have the same name
    fn check(self) -> Result<Self, Box<dyn Error>> {
        // Remove duplicate logins
        let logins = utils::remove_duplicates::<Login>(self.logins);

        if utils::has_duplicates_by_key(&self.containers, |c| c.name.clone()) {
            return Err("Duplicate container names".into());
        }
        Ok(Config { logins, ..self })
    }
}

impl ContainerConfig {
    pub fn base_directory(&self) -> PathBuf {
        self.base_directory
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or(PathBuf::from(".")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::ToRelative;

    #[tokio::test]
    async fn test_parse() {
        let config = include_str!("../../tests/example.toml");
        let config = Config::from_str(config).unwrap();
        assert_eq!(config.containers.len(), 2);
        assert_eq!(config.containers[0].name, "container-cubby");
        assert_eq!(
            config.containers[0].image,
            "cubby.no/rosvik/container-cubby:main"
        );
        assert_eq!(config.containers[0].network, Some(String::from("cubby")));
        assert_eq!(config.containers[1].name, "hello");
    }

    #[tokio::test]
    async fn test_include() {
        let config = include_str!("../../tests/include1.toml");
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

        // Find container with name "qr.248.no"
        let qr_248_no = config
            .containers
            .iter()
            .find(|c| c.name == "qr.248.no")
            .unwrap();

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
