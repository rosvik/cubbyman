use bollard::{auth::DockerCredentials, image::CreateImageOptions};
use futures::StreamExt;
use std::default::Default;
use std::env;

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
