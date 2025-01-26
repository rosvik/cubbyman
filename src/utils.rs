use bollard::secret::{Port, PortTypeEnum};

pub fn bytes_to_human(bytes: i64) -> String {
    let gb: f32 = bytes as f32 / 1024.0 / 1024.0 / 1024.0;
    let mb: f32 = bytes as f32 / 1024.0 / 1024.0;
    let kb: f32 = bytes as f32 / 1024.0;
    if gb > 1.0 {
        format!("{:.2} GB", gb)
    } else if mb > 1.0 {
        format!("{:.2} MB", mb)
    } else if kb > 1.0 {
        format!("{:.2} KB", kb)
    } else {
        format!("{:.2} B", bytes)
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

use base64::{engine::general_purpose, Engine as _};
pub fn decode_base64(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = general_purpose::STANDARD.decode(input)?;
    let utf8 = std::str::from_utf8(&bytes)?;
    Ok(utf8.to_string())
}
