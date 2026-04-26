use bollard::secret::{Port, PortTypeEnum};
use clio::Input;
use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

pub fn has_duplicates_by_key<T, K: Eq + Hash>(vec: &[T], key: fn(&T) -> K) -> bool {
    let mut map = HashMap::new();
    for item in vec.iter() {
        map.insert(key(item), true);
    }
    map.len() != vec.len()
}

pub fn remove_duplicates<T: Eq + Hash>(vec: Vec<T>) -> Vec<T> {
    let mut map = HashMap::new();
    for item in vec {
        map.insert(item, true);
    }
    map.into_keys().collect()
}

pub fn get_image_registry(image: &str) -> String {
    let first_part = image.split('/').next().unwrap().to_string();
    if first_part.contains(".") {
        first_part
    } else {
        String::from("docker.io")
    }
}

pub fn bytes_to_human(bytes: i64) -> String {
    let gb: f32 = bytes as f32 / 1024.0 / 1024.0 / 1024.0;
    let mb: f32 = bytes as f32 / 1024.0 / 1024.0;
    let kb: f32 = bytes as f32 / 1024.0;
    if gb > 1.0 {
        format!("{gb:.2} GB")
    } else if mb > 1.0 {
        format!("{mb:.2} MB")
    } else if kb > 1.0 {
        format!("{kb:.2} KB")
    } else {
        format!("{bytes} B")
    }
}

pub fn format_port(port: &Port) -> String {
    let url_prefix = match port.typ {
        Some(PortTypeEnum::TCP) => "http://",
        Some(PortTypeEnum::UDP) => "(UDP) ",
        Some(PortTypeEnum::SCTP) => "(SCTP) ",
        _ => "",
    };
    format!(
        "{} -> {}{}:{:?}",
        port.private_port,
        url_prefix,
        port.ip.clone().unwrap_or_default(),
        port.public_port.unwrap_or_default()
    )
}

use base64::{Engine as _, engine::general_purpose};
pub fn decode_base64(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = general_purpose::STANDARD.decode(input)?;
    let utf8 = std::str::from_utf8(&bytes)?;
    Ok(utf8.to_string())
}

pub fn load_env_in(
    directory: &Path,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let env_file = directory.join(".env");
    let env_file = File::open(env_file)?;
    let env_file = BufReader::new(env_file);
    parse_env_file(env_file)
}
fn parse_env_file(
    file: BufReader<File>,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut env = HashMap::new();
    for line in file.lines() {
        let line = line?;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line.split_once('=').ok_or("Invalid .env file")?;
        env.insert(key.to_string(), value.to_string());
    }
    Ok(env)
}

pub fn config_path_or_default(input: Option<Input>) -> Option<PathBuf> {
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

    #[test]
    fn test_get_image_registry() {
        assert_eq!(get_image_registry("docker.io/ubuntu"), "docker.io");
        assert_eq!(get_image_registry("cubby.no/hello-world"), "cubby.no");
        assert_eq!(get_image_registry("ghcp.no/rosvik/cve.248.no"), "ghcp.no");
        assert_eq!(get_image_registry("postgres"), "docker.io");
    }
}
