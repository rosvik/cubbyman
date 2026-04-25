use crate::{
    config::ContainerConfig,
    traits::{ToAbsolute, ToRelative, ToString},
    utils::*,
};
use bollard::{
    container::{CreateContainerOptions, ListContainersOptions, RemoveContainerOptions},
    secret::{HostConfig, MountTypeEnum, PortBinding},
};
use crossterm::style::Stylize;
use std::{collections::HashMap, default::Default};

pub async fn run_container(socket: &bollard::Docker, config: ContainerConfig) {
    println!("Running image {} ({})", config.image, config.name);

    let base_directory = config.base_directory();

    let options = CreateContainerOptions::<String> {
        name: config.name.clone(),
        ..Default::default()
    };
    let port_bindings = config
        .ports
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|port| {
            (
                format!("{}/tcp", port.container),
                Some(vec![PortBinding {
                    host_ip: Some("0.0.0.0".to_string()),
                    host_port: Some(port.host.to_string()),
                }]),
            )
        })
        .collect();
    let mut mounts: Vec<bollard::secret::Mount> = config
        .mounts
        .unwrap_or_default()
        .iter()
        .map(|mount| bollard::secret::Mount {
            target: Some(mount.container_path.clone()),
            source: Some(
                mount
                    .host_path
                    .to_relative(&base_directory)
                    .to_absolute()
                    .to_string(),
            ),
            typ: Some(MountTypeEnum::BIND),
            ..Default::default()
        })
        .collect();
    config
        .volumes
        .unwrap_or_default()
        .iter()
        .for_each(|volume| {
            mounts.push(bollard::secret::Mount {
                source: Some(volume.name.clone()),
                target: Some(volume.container_path.clone()),
                typ: Some(MountTypeEnum::VOLUME),
                ..Default::default()
            })
        });

    let host_config = HostConfig {
        network_mode: config.network,
        port_bindings: Some(port_bindings),
        mounts: Some(mounts),
        ..Default::default()
    };
    let empty = HashMap::<(), ()>::new();
    let mut exposed_ports = HashMap::new();
    for port in config.ports.unwrap_or_default() {
        let exposed_port = format!("{}/tcp", port.container);
        exposed_ports.insert(exposed_port, empty.clone());
    }
    let env: Vec<String> = config.env.iter().map(|e| e.to_string()).collect();
    let bollard_config = bollard::container::Config::<String> {
        image: Some(config.image),
        cmd: config.cmd,
        env: (!env.is_empty()).then_some(env),
        host_config: Some(host_config),
        exposed_ports: Some(exposed_ports),
        user: config.user,
        ..Default::default()
    };

    // Attempt to remove container in case it already exists
    if remove_container(socket, config.name.clone()).await.is_ok() {
        println!("Removed pre-existing container {}", config.name.clone())
    }

    match socket.create_container(Some(options), bollard_config).await {
        Ok(container) => {
            println!("Created container {}", container.id.dark_cyan());
            container.warnings.iter().for_each(|warning| {
                println!("Warning: {}", warning.clone().red());
            });
        }
        Err(e) => {
            println!("Error creating container: {e}");
            return;
        }
    }

    match socket
        .start_container::<String>(config.name.as_str(), None)
        .await
    {
        Ok(()) => println!("{}", "Container started".dark_green()),
        Err(e) => println!("Error starting container: {}", format!("{e:?}").dark_red()),
    }
}

pub async fn remove_container(
    socket: &bollard::Docker,
    container_name: String,
) -> Result<(), bollard::errors::Error> {
    let options = RemoveContainerOptions {
        force: true,
        ..Default::default()
    };
    socket
        .remove_container(container_name.as_str(), Some(options))
        .await
}

pub async fn print_containers(socket: &bollard::Docker) {
    let options = ListContainersOptions::<String> {
        ..Default::default()
    };

    let containers = socket
        .list_containers::<String>(Some(options))
        .await
        .unwrap();
    println!();
    containers.iter().for_each(|container| {
        if let Some(names) = &container.names {
            names.iter().for_each(|name| {
                print!("{} ", &name[1..].cyan());
            });
        }
        if let Some(image) = &container.image {
            print!("{} ", image.clone().dark_cyan());
        }
        if let Some(id) = &container.id {
            println!("{}", &id[..12].dark_grey());
        }

        if let Some(status) = &container.status {
            print!("{} ", status.clone().yellow());
        }
        container.ports.iter().for_each(|port| {
            port.iter().for_each(|p| {
                println!("{} ", format_port(p).dark_yellow());
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
