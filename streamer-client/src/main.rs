mod init;
mod input;
use std::collections::HashMap;

use enigo::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let settings = init::init();

    let client = reqwest::Client::new();
    let mut enigo = Enigo::new();
    let mut auth = HashMap::new();
    auth.insert("api_token", &settings.api_token);
    auth.insert("org_id", &settings.org_id);

    loop {
        let res = client
            .post("http://localhost:3000/shift")
            .json(&auth)
            .send()
            .await
            .unwrap();

        match res.status() {
            reqwest::StatusCode::OK => match res.json::<input::Input>().await {
                Ok(parsed) => {
                    if !parsed.error.is_some() {
                        let parsed = parsed.body.expect("Missing input");
                        if parsed.len() != 0 {
                            for ele in parsed {
                                println!("{:?}", ele);
                                input::click(&ele, &mut enigo, &settings);
                            }
                        } else {
                            println!("Waiting for inputs to be sent to the server.");
                        }
                    } else {
                        panic!(
                            "\nError connecting to the server. Server returned \n`{}`",
                            parsed.error.unwrap()
                        )
                    }
                }
                Err(err) => println!("Response did not match type of Input. \n`{:?}`", err),
            },
            _other => {
                panic!("Response was not ok")
            }
        }

        sleep(Duration::from_millis(settings.poll_rate)).await;
    }
}
