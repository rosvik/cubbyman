use crate::utils::*;

pub async fn print_images(connection: bollard::Docker) {
    let images = connection.list_images::<String>(None).await.unwrap();
    images.iter().for_each(|image| {
        println!("ID: {}", image.id);
        image
            .repo_tags
            .iter()
            .for_each(|tag| println!("-> {}", tag));
        println!("\tLabels: {:?}", image.labels);
        // println!("\tManifests: {:?}", image.manifests);
        println!("\tParent ID: {:?}", image.parent_id);
        println!("\tShared Size: {}", bytes_to_human(image.shared_size));
        println!("\tSize: {}", bytes_to_human(image.size));
        println!(
            "\tVirtual Size: {}",
            bytes_to_human(image.virtual_size.unwrap_or(0))
        );
        println!("\tRepo Digests: {:?}", image.repo_digests);
    });
}
