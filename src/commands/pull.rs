use bollard::{auth::DockerCredentials, image::CreateImageOptions};
use futures::StreamExt;
use std::default::Default;
use std::env;

pub async fn pull_image(connection: bollard::Docker, image: String) {
    println!("Pulling image {}", image);
    let options = CreateImageOptions::<String> {
        from_image: image,
        ..Default::default()
    };
    let credentials = DockerCredentials {
        username: Some(env::var("REGISTRY_USERNAME").unwrap()),
        password: Some(env::var("REGISTRY_PASSWORD").unwrap()),
        ..Default::default()
    };

    let mut result = connection.create_image(Some(options), None, Some(credentials));

    while let Some(next) = result.next().await {
        println!("{:?}", next);
    }
}
