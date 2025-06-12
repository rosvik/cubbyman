use crate::utils::*;
use bollard::image::RemoveImageOptions;
use bollard::{auth::DockerCredentials, image::CreateImageOptions};
use futures::StreamExt;
use std::default::Default;
use std::env;
use std::io::Write;

pub async fn print_images(socket: &bollard::Docker) {
    let images = socket.list_images::<String>(None).await.unwrap();
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

pub async fn pull_image(socket: &bollard::Docker, image: String) {
    println!("Pulling image {}", image);

    let credentials = get_credentials();
    let options = CreateImageOptions::<String> {
        from_image: image,
        ..Default::default()
    };

    let mut result = socket.create_image(Some(options), None, credentials);

    while let Some(Ok(create_image_info)) = result.next().await {
        println!("{:?}", create_image_info);
    }
}

pub async fn delete_image(socket: &bollard::Docker, image: &str) {
    println!("Deleting image {}", image);

    let credentials = get_credentials();
    let options = RemoveImageOptions {
        force: false,
        ..Default::default()
    };

    let result = socket.remove_image(image, Some(options), credentials).await;

    println!("{:?}", result);
}

fn get_credentials() -> Option<DockerCredentials> {
    if let Ok(username) = env::var("REGISTRY_USERNAME") {
        if let Ok(password) = env::var("REGISTRY_PASSWORD") {
            return Some(DockerCredentials {
                username: Some(username),
                password: Some(password),
                ..Default::default()
            });
        }
    }
    None
}
