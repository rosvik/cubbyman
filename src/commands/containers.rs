use bollard::{
    container::{Config, CreateContainerOptions, ListContainersOptions, RemoveContainerOptions},
    secret::{HostConfig, PortBinding},
};
use crossterm::style::Stylize;
use std::{collections::HashMap, default::Default};

pub async fn run_container(connection: bollard::Docker, image: String) {
    println!("Running image {}", image);

    let name = "container-cubby".to_string();

    let options = CreateContainerOptions::<String> {
        name: name.clone(),
        ..Default::default()
    };

    let port_bindings = HashMap::from([(
        "8602/tcp".to_string(),
        Some(vec![PortBinding {
            host_ip: Some("127.0.0.1".to_string()),
            host_port: Some("8602".to_string()),
        }]),
    )]);

    let config = Config::<String> {
        image: Some(image),
        env: Some(vec![
            "USERNAME=admin".to_string(),
            "PASSWORD=hunter2".to_string(),
            "PORT=8602".to_string(),
            "HOST=0.0.0.0".to_string(),
        ]),
        host_config: Some(HostConfig {
            port_bindings: Some(port_bindings),
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = connection.create_container(Some(options), config).await;

    println!("Created container");
    println!("{:?}", result);

    let result = connection
        .start_container::<String>(name.as_str(), None)
        .await;

    println!("Started container");
    println!("{:?}", result);
}

pub async fn remove_container(connection: bollard::Docker, container: String) {
    println!("Removing container {}", container);

    let options = RemoveContainerOptions {
        force: true,
        ..Default::default()
    };

    let result = connection
        .remove_container(container.as_str(), Some(options))
        .await;
    println!("Removed container");
    println!("{:?}", result);
}

pub async fn print_containers(connection: bollard::Docker) {
    let options = ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    };

    let containers = connection
        .list_containers::<String>(Some(options))
        .await
        .unwrap();
    containers.iter().for_each(|container| {
        if let Some(id) = &container.id {
            println!("{}", id.clone().grey());
        }
        if let Some(image) = &container.image {
            print!("{}", image.clone().dark_cyan());
        }
        if let Some(names) = &container.names {
            print!(" ( ");
            names.iter().for_each(|name| {
                print!("{} ", name.clone().dark_cyan());
            });
            print!(")");
        }

        if let Some(status) = &container.status {
            print!(" {}", status.clone().yellow());
        }
        println!();

        if let Some(labels) = &container.labels {
            labels.iter().for_each(|(key, label)| {
                println!("\t{}: {}", key.clone().dark_grey(), label.clone().grey());
            });
        }
        println!();
    });
}
