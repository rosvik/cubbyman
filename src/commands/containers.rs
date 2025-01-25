use crate::{config::Config, utils::*};
use bollard::{
    container::{CreateContainerOptions, ListContainersOptions, RemoveContainerOptions},
    secret::{HostConfig, PortBinding},
};
use crossterm::style::Stylize;
use std::default::Default;

pub async fn run_container(connection: &bollard::Docker, config: Config) {
    println!("Running image {} ({})", config.image, config.name);

    let options = CreateContainerOptions::<String> {
        name: config.name.clone(),
        ..Default::default()
    };
    let port_bindings = config
        .ports
        .iter()
        .map(|port| {
            (
                format!("{}/tcp", port.container),
                Some(vec![PortBinding {
                    host_ip: Some("127.0.0.1".to_string()),
                    host_port: Some(port.host.to_string()),
                }]),
            )
        })
        .collect();
    let bollard_config = bollard::container::Config::<String> {
        image: Some(config.image),
        env: Some(config.env),
        host_config: Some(HostConfig {
            port_bindings: Some(port_bindings),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Attempt to remove container in case it already exists
    let result = remove_container(connection, config.name.clone()).await;
    match result {
        Ok(()) => println!("Removed pre-existing container {}", config.name.clone()),
        Err(e) => println!("Error removing container: {}", e),
    }

    let result = connection
        .create_container(Some(options), bollard_config)
        .await;

    println!("Created container");
    println!("{:?}", result);

    let result = connection
        .start_container::<String>(config.name.as_str(), None)
        .await;

    println!("Started container");
    println!("{:?}", result);
}

pub async fn remove_container(
    connection: &bollard::Docker,
    container: String,
) -> Result<(), bollard::errors::Error> {
    let options = RemoveContainerOptions {
        force: true,
        ..Default::default()
    };
    connection
        .remove_container(container.as_str(), Some(options))
        .await
}

pub async fn print_containers(connection: &bollard::Docker) {
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
            println!("{}", id.clone().dark_grey());
        }
        if let Some(image) = &container.image {
            print!("{} ", image.clone().dark_cyan());
        }
        if let Some(names) = &container.names {
            names.iter().for_each(|name| {
                print!("{} ", name.clone().cyan());
            });
        }

        println!();

        if let Some(status) = &container.status {
            print!("{} ", status);
        }
        container.ports.iter().for_each(|port| {
            port.iter().for_each(|p| {
                println!("{} ", format_port(p).yellow());
            });
        });

        if let Some(labels) = &container.labels {
            labels.iter().for_each(|(key, label)| {
                println!("\t{}: {}", key.clone().dark_grey(), label.clone().grey());
            });
        }
        println!();
    });
}
