mod init;
mod input;
use enigo::*;
use reqwest::header::CONTENT_TYPE;
use std::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let settings = init::init();

    let client = reqwest::Client::new();
    let mut enigo = Enigo::new();

    let res = client
        .get(&settings.url)
        .header("AUTH", &settings.auth)
        .header(CONTENT_TYPE, "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .unwrap();

    match res.status() {
        reqwest::StatusCode::OK => match res.json::<input::Input>().await {
            Ok(parsed) => {
                println!("{:?}", parsed);
                input::click(&parsed.input[..], &mut enigo, &settings);
            }
            Err(err) => println!("Response did not match type of Input. \n`{:?}`", err),
        },
        _other => {
            panic!("Response from `{:?}` was not ok", &settings.url)
        }
    }

    Ok(())
}
