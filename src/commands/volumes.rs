use bollard::volume::CreateVolumeOptions;

pub async fn create_volume(socket: &bollard::Docker, name: String) {
    // Abort if volume already exists
    if socket.inspect_volume(&name).await.is_ok() {
        println!("Volume {} already exists", name);
        return;
    }

    let options = CreateVolumeOptions::<String> {
        name,
        ..Default::default()
    };
    match socket.create_volume(options).await {
        Ok(volume) => println!("Created volume {}", volume.name),
        Err(e) => println!("Error creating volume: {}", e),
    }
}
