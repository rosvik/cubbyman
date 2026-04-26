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
