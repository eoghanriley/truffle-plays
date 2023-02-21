use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    // Top dpad
    pub d1_left: char,
    pub d1_up: char,
    pub d1_right: char,
    pub d1_down: char,

    // Top buttons
    pub x1: char,
    pub a1: char,
    pub b1: char,
    pub y1: char,

    // Bottom dpad
    pub d2_left: char,
    pub d2_up: char,
    pub d2_right: char,
    pub d2_down: char,

    // Bottom buttons
    pub x2: char,
    pub a2: char,
    pub b2: char,
    pub y2: char,

    // Password to server GET
    pub auth: String,
    // rate to poll server in milliseconds
    pub poll_rate: u64,
    pub url: String,
}

pub fn init() -> Settings {
    let settings: Settings = {
        let data = fs::read_to_string("./settings.json").expect("Error reading settings.json");
        serde_json::from_str(&data).unwrap()
    };

    settings
}
