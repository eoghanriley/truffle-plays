mod init;
mod input;
use enigo::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let settings = init::init();

    let client = reqwest::Client::new();
    let mut enigo = Enigo::new();

    loop {
        let res = client
            .get(&settings.url)
            .header("AUTH", &settings.auth)
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

        sleep(Duration::from_millis(settings.poll_rate)).await;
    }
}
