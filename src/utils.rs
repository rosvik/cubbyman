use bollard::secret::Port;

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
    format!(
        "{} -> {}:{:?}",
        port.private_port,
        port.ip.clone().unwrap_or_default(),
        port.public_port.unwrap_or_default()
    )
}
