mod init;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Deserialize, Serialize, Debug)]
struct Input {
    input: String,
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let settings = init::init();

    let client = reqwest::Client::new();

    let res = client
        .get(&settings.url)
        .header("AUTH", settings.auth)
        .header(CONTENT_TYPE, "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .unwrap();

    match res.status() {
        reqwest::StatusCode::OK => match res.json::<Input>().await {
            Ok(parsed) => println!("{:?}", parsed),
            Err(err) => println!("Response did not match type of Input {:?}", err),
        },
        _other => {
            panic!("Response from {:?} was not ok", &settings.url)
        }
    }

    Ok(())
}
