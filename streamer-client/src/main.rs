use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Deserialize, Serialize, Debug)]
struct Settings {
    // Top dpad
    d1_left: char,
    d1_up: char,
    d1_right: char,
    d1_down: char,

    // Top buttons
    x1: char,
    a1: char,
    b1: char,
    y1: char,

    // Bottom dpad
    d2_left: char,
    d2_up: char,
    d2_right: char,
    d2_down: char,

    // Bottom buttons
    x2: char,
    a2: char,
    b2: char,
    y2: char,
}

fn init() -> Settings {
    let settings: Settings = {
        let data = fs::read_to_string("./settings.json").expect("Error reading settings.json");
        serde_json::from_str(&data).unwrap()
    };

    settings
}

fn main() -> Result<(), io::Error> {
    println!("{:?}", init());

    Ok(())
}
