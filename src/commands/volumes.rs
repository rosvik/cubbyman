use bollard::volume::{CreateVolumeOptions, RemoveVolumeOptions};

#[allow(dead_code)]
pub async fn create_volume(socket: &bollard::Docker, name: &String) {
    let options = CreateVolumeOptions::<String> {
        name: String::from(name),
        ..Default::default()
    };
    match socket.create_volume(options).await {
        Ok(_) => println!("Volume {} created", name),
        Err(e) => println!("Error creating volume: {}", e),
    }
}

#[allow(dead_code)]
pub async fn remove_volume(socket: &bollard::Docker, name: &str) {
    let options = RemoveVolumeOptions { force: true };
    match socket.remove_volume(name, Some(options)).await {
        Ok(_) => println!("Volume {} removed", name),
        Err(e) => println!("Error removing volume: {}", e),
    }
}

pub async fn list_volumes(socket: &bollard::Docker) {
    let volumes = socket.list_volumes::<String>(None).await.unwrap().volumes;
    volumes.unwrap().iter().for_each(|volume| {
        print!("Volume '{}', ", volume.name);
        if let Some(usage_data) = &volume.usage_data {
            print!("{} bytes, ", usage_data.size);
        }
        println!();
    });
}
