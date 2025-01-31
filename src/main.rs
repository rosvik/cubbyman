use clap::Parser;

mod commands;
mod config;
mod middleware;
mod serve;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, help = "Print status of the current connection")]
    status: bool,
    #[arg(long, help = "Serve webhooks")]
    serve: Option<String>,

    #[arg(long, help = "List all images")]
    list_images: bool,
    #[arg(long, help = "List all containers")]
    list_containers: bool,

    #[arg(long, help = "Pull an image")]
    pull: Option<String>,
    #[arg(
        long,
        help = "Setup and run containers using the specified config file"
    )]
    run: Option<String>,
    #[arg(long, help = "Destroy containers listed in the specified config file")]
    destroy: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let socket = bollard::Docker::connect_with_local_defaults().unwrap();
    let version = socket.version().await.unwrap();
    let args = Args::parse();

    if args.status {
        commands::system::print_status(version);
        return;
    }
    if let Some(config_path) = args.serve {
        let config = config::load_config_from_file(&config_path).unwrap();
        serve::serve(socket, config).await;
        return;
    }

    if let Some(image) = args.pull {
        commands::images::pull_image(&socket, image).await;
    } else if let Some(config_path) = args.run {
        let config = config::load_config_from_file(&config_path).unwrap();
        commands::system::reload_all(&socket, &config).await;
    } else if let Some(config_path) = args.destroy {
        let config = config::load_config_from_file(&config_path).unwrap();
        for container in config.containers.iter() {
            commands::containers::remove_container(&socket, container.name.clone())
                .await
                .unwrap();
        }
    }

    if args.list_images {
        commands::images::print_images(&socket).await;
    }
    if args.list_containers {
        commands::containers::print_containers(&socket).await;
    }
}
