use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct IpfsID {
    id: String,
}

#[derive(Deserialize)]
struct ClusterID {
    id: String,
}

// Function to get IP address
pub async fn get_ip_address() -> Result<String, Box<dyn Error>> {
    let res = reqwest::get("https://httpbin.org/ip").await?.text().await?;
    let json: Value = serde_json::from_str(&res)?;

    if let Some(ip) = json.get("origin").and_then(Value::as_str) {
        Ok(ip.to_string())
    } else {
        Err("Unable to extract IP address".into())
    }
}

// Function to get IPFS ID
pub async fn get_ipfs_id(ip_address: Option<&str>) -> Result<String, Box<dyn Error>> {
    let url = if let Some(ip) = ip_address {
        format!("http://{}:5001/api/v0/id", ip)
    } else {
        "http://127.0.0.1:5001/api/v0/id".to_string()
    };

    let client = Client::new();
    println!("client {:?}", client);
    let res = client.post(&url).json(&()).send().await?;
    println!("res {:?}", res);
    let ipfs_id: IpfsID = res.json().await?;
    println!("ipfs_id {:?}", ipfs_id);

    Ok(ipfs_id.id)
}

// Function to get Cluster ID
pub async fn get_cluster_id(ip_address: Option<&str>) -> Result<String, Box<dyn Error>> {
    let url = if let Some(ip) = ip_address {
        format!("http://{}:9094/id", ip)
    } else {
        "http://127.0.0.1:9094/id".to_string()
    };

    let client = Client::new();
    let res = client.get(&url).send().await?;
    let cluster_id: ClusterID = res.json().await?;

    Ok(cluster_id.id)
}
