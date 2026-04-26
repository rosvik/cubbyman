use crate::traits::ToString;
use serde::{Deserialize, Deserializer, Serialize};
use std::{fmt::Display, path::PathBuf};

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
pub fn deserialize_mounts<'de, D>(deserializer: D) -> Result<Option<Vec<Mount>>, D::Error>
where
    D: Deserializer<'de>,
{
    let mounts: Option<Vec<String>> = Deserialize::deserialize(deserializer)?;
    Ok(mounts.map(|mounts| mounts.iter().map(|m| Mount::from_string(m)).collect()))
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, traits::ToRelative};
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_path_after_include() {
        let config = Config::load(&PathBuf::from("tests/include1.toml")).unwrap();

        let qr_248_no = config
            .containers
            .iter()
            .find(|c| c.name == "qr.248.no")
            .unwrap();

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
