use clap::Parser;
use clio::Input;

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
    serve: Option<Option<Input>>,

    #[arg(long, help = "List all images")]
    list_images: bool,
    #[arg(long, help = "List all containers")]
    list_containers: bool,
    #[arg(long, help = "List all networks")]
    list_networks: bool,
    #[arg(long, help = "List all volumes")]
    list_volumes: bool,

    #[arg(
        long,
        help = "Setup and run containers using the specified config file"
    )]
    apply: Option<Option<Input>>,
    #[arg(long, help = "Destroy containers listed in the specified config file")]
    destroy: Option<Option<Input>>,

    #[arg(
        long,
        help = "Print the contents of the current, or provided config file"
    )]
    print_config: Option<Option<Input>>,
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
    if let Some(input_file) = args.serve {
        let config = config::load_config(input_file).unwrap();
        serve::serve(socket, config).await;
        return;
    }
    if let Some(input_file) = args.print_config {
        let config = config::load_config(input_file).unwrap();
        println!("{}", toml::to_string(&config).unwrap());
        return;
    }

    if let Some(input_file) = args.apply {
        let config = config::load_config(input_file).unwrap();
        commands::system::reload_all(&socket, &config).await;
    } else if let Some(input_file) = args.destroy {
        let config = config::load_config(input_file).unwrap();
        for container in config.containers.iter() {
            commands::containers::remove_container(&socket, container.name.clone())
                .await
                .unwrap();
            if let Some(network) = &container.network {
                commands::networks::remove_network(&socket, network).await;
            }
        }
    }

    if args.list_images {
        commands::images::print_images(&socket).await;
    }
    if args.list_containers {
        commands::containers::print_containers(&socket).await;
    }
    if args.list_networks {
        commands::networks::list_networks(&socket).await;
    }
    if args.list_volumes {
        commands::volumes::list_volumes(&socket).await;
    }
}
