use crate::utils::*;
use bollard::{auth::DockerCredentials, image::CreateImageOptions};
use futures::StreamExt;
use std::default::Default;
use std::env;

pub async fn print_images(connection: bollard::Docker) {
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

pub async fn pull_image(connection: bollard::Docker, image: String) {
    println!("Pulling image {}", image);

    let credentials = get_credentials();
    let options = CreateImageOptions::<String> {
        from_image: image,
        ..Default::default()
    };

    let mut result = connection.create_image(Some(options), None, Some(credentials));

    while let Some(Ok(create_image_info)) = result.next().await {
        println!("{:?}", create_image_info);
    }
}

fn get_credentials() -> DockerCredentials {
    DockerCredentials {
        username: Some(env::var("REGISTRY_USERNAME").unwrap()),
        password: Some(env::var("REGISTRY_PASSWORD").unwrap()),
        ..Default::default()
    }
}
