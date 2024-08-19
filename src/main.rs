use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::path::PathBuf;
use tokio::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    url: String,
    ip: String,
    api_key: String,
}

#[derive(Serialize, Debug, Clone, Default)]
struct UpdatePayload {
    content: String,
    name: String,
    proxied: Option<bool>,
    r#type: String,
    comment: Option<String>,
    tags: Option<Vec<String>>,
    ttl: Option<i16>,
}

#[derive(Deserialize, Debug)]
struct UpdateResponseMessage {
    code: u64,
    message: String,
}
#[derive(Deserialize, Debug)]
struct UpdateResponseResultMeta {
    auto_added: Option<bool>,
    source: Option<String>,
}
#[derive(Deserialize, Debug)]
struct UpdateResponseResult {
    content: String,
    name: String,
    proxied: Option<bool>,
    r#type: String,
    comment: Option<String>,
    created_on: String,
    id: String,
    locked: bool,
    meta: Option<UpdateResponseResultMeta>,
    modified_on: String,
    proxiable: bool,
    tags: Option<Vec<String>>,
    ttl: Option<u64>,
    zone_id: Option<String>,
    zone_name: String,
}

#[derive(Deserialize, Debug)]
struct UpdateResponse {
    errors: Vec<UpdateResponseMessage>,
    messages: Vec<UpdateResponseMessage>,
    success: bool,
    result: UpdateResponseResult,
}

async fn get_config() -> Result<Config, serde_json::Error> {
    let path = PathBuf::from("./config.json");
    let contents = fs::read_to_string(path).await.expect("File is corrupt");

    serde_json::from_str(contents.as_str())
}

async fn update_config(mut config: Config, new_ip: String) {
    println!("Writing new IP to config.");
    config.ip = new_ip;
    fs::write(
        PathBuf::from("./config.json"),
        serde_json::to_string::<Config>(&config).expect("Unable to serialize config."),
    )
    .await
    .expect("Unable to write JSON file.");
}

async fn get_current_ip() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config().await?;

    println!("Attempting bind.");

    let current_ip = reqwest::Client::builder()
        .local_address(IpAddr::from([0, 0, 0, 0]))
        .build()?
        .get("https://ifconfig.me")
        .send()
        .await?
        .text()
        .await?;

    if config.ip == current_ip {
        println!("No change.");
        Ok(())
    } else {
        post_updated_ip(config, current_ip).await
    }
}

async fn post_updated_ip(config: Config, new_ip: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("IP changed! {} -> {}", config.ip, new_ip);

    let json_payload = UpdatePayload {
        content: new_ip.clone(),
        name: "home".to_owned(),
        r#type: "A".to_owned(),
        comment: Some(eos::DateTime::utc_now().to_string()),
        ..Default::default()
    };

    let conf = config.clone();

    println!("Pushing new IP.");
    let response = reqwest::Client::new()
        .patch(conf.url)
        .header("Authorization", format!("Bearer {}", conf.api_key))
        .json::<UpdatePayload>(&json_payload)
        .send()
        .await?
        .json::<UpdateResponse>()
        .await?;
    println!("Pushed new IP.");

    update_config(config, response.result.content).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        get_current_ip().await?;
        println!("Finished iteration, sleeping for 5m");
        tokio::time::sleep(std::time::Duration::from_secs(300)).await;
    }
}
