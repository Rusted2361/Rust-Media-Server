use actix_web::{HttpResponse, Responder};
use serde_json::json;
use crate::helpers::helpers::{get_ip_address, get_ipfs_id, get_cluster_id};

// Actual handler function
pub async fn get_status() -> impl Responder {
    print!("welcome to status route");
    // Get IP address
    let ip_address = match get_ip_address().await {
        Ok(ip) => ip,
        Err(err) => {
            println!("Error fetching IP address: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch IP address"}));
        }
    };
    println!("ipaddress: {}", ip_address);

    // Get IPFS ID
    let ipfs_id = match get_ipfs_id(Some(&ip_address)).await {
        Ok(id) => id,
        Err(err) => {
            println!("Error fetching IPFS ID: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch IPFS ID"}));
        }
    };
    println!("ipfsid: {}", ipfs_id);

    // Get cluster ID
    let cluster_id = match get_cluster_id(None).await {
        Ok(id) => id,
        Err(err) => {
            println!("Error fetching Cluster ID: {}", err);
            return HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch Cluster ID"}));
        }
    };
    println!("clusterid: {}", cluster_id);

    // Check if cluster is online
    let is_cluster_online = !cluster_id.is_empty();

    // Send JSON response
    HttpResponse::Ok().json(json!({
        "isClusterOnline": is_cluster_online
    }))
}
