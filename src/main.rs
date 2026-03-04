mod models;
use crate::models::serde::{Config, IPResponsePayload, UpdatePayload, UpdateResponse};
use std::net::IpAddr;
use std::path::PathBuf;
use tokio::fs;

fn flush_stdout() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char)
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

    flush_stdout();

    println!(
        "Attempting bind at {}.",
        eos::DateTime::utc_now().to_rfc3339()
    );

    let resp = reqwest::Client::builder()
        .local_address(IpAddr::from([0, 0, 0, 0]))
        .build()?
        .get("https://api.ipify.org?format=json")
        .send()
        .await?
        .json::<IPResponsePayload>()
        .await?;

    if config.ip == resp.ip {
        println!("No change.");
        Ok(())
    } else {
        post_updated_ip(config, resp.ip).await
    }
}

async fn post_updated_ip(config: Config, new_ip: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("IP changed! {} -> {}", config.ip, new_ip);

    let json_payload = UpdatePayload {
        content: new_ip.clone(),
        name: "home".to_owned(),
        r#type: "A".to_owned(),
        comment: Some(eos::DateTime::utc_now().to_string()),
        proxied: Some(true),
        ..Default::default()
    };

    let conf = config.clone();

    println!("Pushing new IP.");
    for url in conf.urls {
        let response = reqwest::Client::new()
            .patch(url)
            .header("Authorization", format!("Bearer {}", conf.api_key))
            .json::<UpdatePayload>(&json_payload)
            .send()
            .await?
            .json::<UpdateResponse>()
            .await?;

        if !response.success {
            println!("There were errors in this request: {:#?}", response.errors);
            panic!("Dying here.")
        }
    }

    update_config(config, new_ip).await;

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
