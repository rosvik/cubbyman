use crate::{
    commands::{containers, images, networks},
    config::Config,
};
use crossterm::style::Stylize;

pub fn print_status(version: bollard::system::Version) {
    if let Some(os) = version.os {
        println!("OS: {}", os.dark_cyan());
    }
    if let Some(arch) = version.arch {
        println!("Arch: {}", arch.dark_cyan());
    }
    if let Some(platform) = version.platform {
        println!("Platform: {}", platform.name.to_string().dark_cyan());
    }
    if let Some(kernel_version) = version.kernel_version {
        println!("Kernel Version: {}", kernel_version.dark_cyan());
    }

    println!(" ");

    if let Some(api_version) = version.api_version {
        println!("API Version: {}", api_version.dark_cyan());
    }
    if let Some(min_api_version) = version.min_api_version {
        println!("Min API Version: {}", min_api_version.dark_cyan());
    }
    if let Some(experimental) = version.experimental {
        println!("Experimental: {}", experimental.dark_cyan());
    }

    println!(" ");

    if let Some(build_time) = version.build_time {
        println!("Build Time: {}", build_time.dark_cyan());
    }
    if let Some(go_version) = version.go_version {
        println!("Go Version: {}", go_version.dark_cyan());
    }
    if let Some(git_commit) = version.git_commit {
        println!("Git Commit: {}", git_commit.dark_cyan());
    }

    println!(" ");

    version.components.iter().for_each(|component| {
        component.iter().for_each(|plugin| {
            println!("Plugin: {}", plugin.name.clone().dark_cyan());
            println!("\t -> Version: {}", plugin.version);
            if let Some(detail) = &plugin.details {
                println!("\t -> Details: {detail:?}");
            }
        });
    });
}

pub async fn reload_all(socket: &bollard::Docker, config: &Config) {
    for container in config.containers.iter() {
        if let Some(network) = &container.network {
            networks::create_network(socket, network).await;
        }
        images::pull_image(socket, config, container.image.clone()).await;
        containers::run_container(socket, container.clone()).await;
    }
    println!("Reloaded all containers");
    containers::print_containers(socket).await;
}

pub async fn reload(
    socket: &bollard::Docker,
    config: &Config,
    name: &str,
) -> Result<(), std::io::Error> {
    if let Some(container) = config.containers.iter().find(|c| c.name == name) {
        if let Some(network) = &container.network {
            networks::create_network(socket, network).await;
        }
        images::pull_image(socket, config, container.image.clone()).await;
        containers::run_container(socket, container.clone()).await;

        Ok(())
    } else {
        Err(std::io::ErrorKind::NotFound.into())
    }
}
