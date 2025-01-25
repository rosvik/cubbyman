use crate::{config::ContainerConfig, utils::*};
use bollard::{
    container::{CreateContainerOptions, ListContainersOptions, RemoveContainerOptions},
    secret::{HostConfig, PortBinding},
};
use crossterm::style::Stylize;
use std::default::Default;

pub async fn run_container(socket: &bollard::Docker, config: ContainerConfig) {
    println!("Running image {} ({})", config.image, config.name);

    let options = CreateContainerOptions::<String> {
        name: config.name.clone(),
        ..Default::default()
    };
    let port_bindings = config
        .ports
        .unwrap_or_default()
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
        env: config.env,
        host_config: Some(HostConfig {
            port_bindings: Some(port_bindings),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Attempt to remove container in case it already exists
    match remove_container(socket, config.name.clone()).await {
        Ok(()) => println!("Removed pre-existing container {}", config.name.clone()),
        Err(e) => println!("Error removing container: {}", e),
    }

    match socket.create_container(Some(options), bollard_config).await {
        Ok(container) => {
            println!("Created container {}", container.id.dark_cyan());
            container.warnings.iter().for_each(|warning| {
                println!("Warning: {}", warning.clone().red());
            });
        }
        Err(e) => {
            println!("Error creating container: {}", e);
            return;
        }
    }

    match socket
        .start_container::<String>(config.name.as_str(), None)
        .await
    {
        Ok(()) => println!("{}", "Container started".dark_green()),
        Err(e) => println!(
            "Error starting container: {}",
            format!("{:?}", e).dark_red()
        ),
    }
}

pub async fn remove_container(
    socket: &bollard::Docker,
    container: String,
) -> Result<(), bollard::errors::Error> {
    let options = RemoveContainerOptions {
        force: true,
        ..Default::default()
    };
    socket
        .remove_container(container.as_str(), Some(options))
        .await
}

pub async fn print_containers(socket: &bollard::Docker) {
    let options = ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    };

    let containers = socket
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
