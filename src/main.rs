#[tokio::main]
async fn main() {
    let connection = bollard::Docker::connect_with_local_defaults().unwrap();
    let version = connection.version().await.unwrap();
    print_connection_info(version);

    let images = connection.list_images::<String>(None).await.unwrap();
    images.iter().for_each(|image| {
        println!("ID: {}", image.id);
        image
            .repo_tags
            .iter()
            .for_each(|tag| println!("-> {}", tag));
        println!("\tLabels: {:?}", image.labels);
        // println!("\tManifests: {:?}", image.manifests);
        println!("\tParent ID: {:?}", image.parent_id);
        println!("\tShared Size: {}", bytes_to_human(image.shared_size));
        println!("\tSize: {}", bytes_to_human(image.size));
        println!(
            "\tVirtual Size: {}",
            bytes_to_human(image.virtual_size.unwrap_or(0))
        );
        println!("\tRepo Digests: {:?}", image.repo_digests);
    });
}

fn print_connection_info(version: bollard::system::Version) {
    if let Some(os) = version.os {
        println!("OS: {}", os);
    }
    version.components.iter().for_each(|component| {
        component.iter().for_each(|plugin| {
            println!("Plugin: {}", plugin.name);
            println!("\t -> Version: {}", plugin.version);
            if let Some(detail) = &plugin.details {
                println!("\t -> Details: {:?}", detail);
            }
        });
    });
    println!("\n");
}

fn bytes_to_human(bytes: i64) -> String {
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
