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
