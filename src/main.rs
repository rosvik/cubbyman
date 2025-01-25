use clap::Parser;

mod commands;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Print status of the current connection")]
    status: bool,

    #[arg(short, long, help = "List all images")]
    list_images: bool,

    #[arg(short, long, help = "Pull an image")]
    pull: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let connection = bollard::Docker::connect_with_local_defaults().unwrap();
    let version = connection.version().await.unwrap();
    let args = Args::parse();

    if args.status {
        commands::system::print_status(version);
    } else if args.list_images {
        commands::images::print_images(connection).await;
    } else if let Some(image) = args.pull {
        commands::images::pull_image(connection, image).await;
    }
}
