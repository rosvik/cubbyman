use serde::{Deserialize, Deserializer, Serialize};

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

pub fn deserialize_secrets<'de, D>(deserializer: D) -> Result<Option<Vec<Secret>>, D::Error>
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config::Config;

    #[tokio::test]
    async fn test_simple_secret() {
        let config = include_str!("../../tests/example.toml");
        let config = Config::from_str(config).unwrap();
        let port = config.containers[0].secrets.as_ref().unwrap()[1].clone();
        assert_eq!(port.dotenv_key, String::from("PORT"));
        assert_eq!(port.container_env_key, String::from("PORT"));
    }

    #[tokio::test]
    async fn test_named_secret() {
        let config = include_str!("../../tests/example.toml");
        let config = Config::from_str(config).unwrap();
        let password = config.containers[0].secrets.as_ref().unwrap()[0].clone();
        assert_eq!(
            password.dotenv_key,
            String::from("CONTAINER_CUBBY_PASSWORD")
        );
        assert_eq!(password.container_env_key, String::from("PASSWORD"));
    }

    #[tokio::test]
    async fn test_env_load() {
        let config = Config::load(&PathBuf::from("tests/example.toml")).unwrap();
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

        let port = container_cubby
            .env
            .iter()
            .find(|e| e.key == "PORT")
            .unwrap();
        assert_eq!(port.value, String::from("8602"));
    }

    #[tokio::test]
    async fn test_env_load_from_include_dir() {
        let config = Config::load(&PathBuf::from("tests/include1.toml")).unwrap();
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
    }
}
