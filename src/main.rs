use clap::Parser;

mod commands;
mod config;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, help = "Print status of the current connection")]
    status: bool,

    #[arg(long, help = "List all images")]
    list_images: bool,
    #[arg(long, help = "Pull an image")]
    pull: Option<String>,

    #[arg(long, help = "List all containers")]
    list_containers: bool,
    #[arg(long, help = "Run a container using the specified config file")]
    run: Option<String>,
    #[arg(long, help = "Remove a container")]
    remove_container: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let socket = bollard::Docker::connect_with_local_defaults().unwrap();
    let version = socket.version().await.unwrap();
    let args = Args::parse();

    if args.status {
        commands::system::print_status(version);
    } else if args.list_images {
        commands::images::print_images(&socket).await;
    } else if let Some(image) = args.pull {
        commands::images::pull_image(&socket, image).await;
    } else if args.list_containers {
        commands::containers::print_containers(&socket).await;
    } else if let Some(config) = args.run {
        let config = config::load_config_from_file(&config).unwrap();
        commands::containers::run_container(&socket, config).await;
    } else if let Some(container) = args.remove_container {
        commands::containers::remove_container(&socket, container)
            .await
            .unwrap();
    }
}
