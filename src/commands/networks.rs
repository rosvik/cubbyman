use bollard::network::CreateNetworkOptions;

pub async fn create_network(socket: &bollard::Docker, name: &String) {
    let options = CreateNetworkOptions {
        name: String::from(name),
        driver: String::from("bridge"),
        ..Default::default()
    };
    match socket.create_network(options).await {
        Ok(_) => println!("Network {name} created",),
        Err(e) => println!("Error creating network: {e}"),
    }
}

pub async fn remove_network(socket: &bollard::Docker, name: &str) {
    match socket.remove_network(name).await {
        Ok(_) => println!("Network {name} removed"),
        Err(e) => println!("Error removing network: {e}"),
    }
}

pub async fn list_networks(socket: &bollard::Docker) {
    let networks = socket.list_networks::<String>(None).await.unwrap();
    networks.iter().for_each(|network| {
        if let Some(name) = &network.name {
            print!("Network '{name}'");
        }
        if let Some(driver) = &network.driver {
            print!("using driver '{driver}'");
        }
        if let Some(id) = &network.id {
            println!("with id '{id}'");
        }
    });
}
