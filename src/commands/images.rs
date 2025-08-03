use crate::config::Config;
use crate::utils::*;
use bollard::image::RemoveImageOptions;
use bollard::{auth::DockerCredentials, image::CreateImageOptions};
use crossterm::style::Stylize;
use futures::StreamExt;
use std::default::Default;

pub async fn print_images(socket: &bollard::Docker) {
    let images = socket.list_images::<String>(None).await.unwrap();
    images.iter().for_each(|image| {
        println!("ID: {}", image.id);
        image.repo_tags.iter().for_each(|tag| println!("-> {tag}"));
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

pub async fn pull_image(socket: &bollard::Docker, config: &Config, image: String) {
    println!("Pulling image {image}");

    let credentials = get_credentials(config, &image);
    let options = CreateImageOptions::<String> {
        from_image: image,
        ..Default::default()
    };

    let mut result = socket.create_image(Some(options), None, credentials);

    while let Some(create_image_info) = result.next().await {
        match create_image_info {
            Ok(create_image_info) => println!("{create_image_info:?}"),
            Err(e) => println!("{}", format!("Error: {e:?}").dark_red()),
        }
    }
}

pub async fn delete_image(socket: &bollard::Docker, config: &Config, image: &str) {
    println!("Deleting image {image}");

    let credentials = get_credentials(config, image);
    let options = RemoveImageOptions {
        force: false,
        ..Default::default()
    };

    let result = socket.remove_image(image, Some(options), credentials).await;

    println!("{result:?}");
}

fn get_credentials(config: &Config, image: &str) -> Option<DockerCredentials> {
    let registry = get_image_registry(image);
    if let Some(logins) = &config.logins {
        let login = logins.iter().find(|login| login.registry == registry);
        login.map(|login| DockerCredentials {
            username: Some(login.username.clone()),
            password: Some(login.password.clone()),
            ..Default::default()
        })
    } else {
        None
    }
}
