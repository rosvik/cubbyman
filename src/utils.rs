use bollard::secret::{Port, PortTypeEnum};
use std::path::PathBuf;

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

pub fn to_absolute_path(path: &str) -> String {
    let mut path = PathBuf::from(path);
    if !path.is_absolute() {
        path = std::env::current_dir().unwrap().join(path);
        path = path.canonicalize().unwrap_or_else(|e| {
            panic!("Failed to resolve path '{}': {}", path.to_string_lossy(), e)
        });
    }
    path.to_string_lossy().to_string()
}

use base64::{Engine as _, engine::general_purpose};
pub fn decode_base64(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = general_purpose::STANDARD.decode(input)?;
    let utf8 = std::str::from_utf8(&bytes)?;
    Ok(utf8.to_string())
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
