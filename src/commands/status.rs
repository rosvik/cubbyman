pub fn print_status(version: bollard::system::Version) {
    if let Some(os) = version.os {
        println!("OS: {}", os);
    }
    version.components.iter().for_each(|component| {
        component.iter().for_each(|plugin| {
            println!("Plugin: {}", plugin.name);
            println!("\t -> Version: {}", plugin.version);
            if let Some(detail) = &plugin.details {
                println!("\t -> Details: {:?}", detail);
            }
        });
    });
}
