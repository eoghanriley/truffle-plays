use crate::init::Settings;
use enigo::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Input {
    pub body: Option<Vec<String>>,
    pub error: Option<String>,
}

pub fn click(input: &str, enigo: &mut Enigo, settings: &Settings) {
    match input {
        // dpad 1
        "d1_left" => enigo.key_click(Key::Layout(settings.d1_left)),
        "d1_up" => enigo.key_click(Key::Layout(settings.d1_up)),
        "d1_right" => enigo.key_click(Key::Layout(settings.d1_right)),
        "d1_down" => enigo.key_click(Key::Layout(settings.d1_down)),

        // Button set 1
        "x1" => enigo.key_click(Key::Layout(settings.x1)),
        "a1" => enigo.key_click(Key::Layout(settings.a1)),
        "b1" => enigo.key_click(Key::Layout(settings.b1)),
        "y1" => enigo.key_click(Key::Layout(settings.y1)),

        // dpad 2
        "d2_left" => enigo.key_click(Key::Layout(settings.d2_left)),
        "d2_up" => enigo.key_click(Key::Layout(settings.d2_up)),
        "d2_right" => enigo.key_click(Key::Layout(settings.d2_right)),
        "d2_down" => enigo.key_click(Key::Layout(settings.d2_down)),

        // Button set 2
        "x2" => enigo.key_click(Key::Layout(settings.x2)),
        "a2" => enigo.key_click(Key::Layout(settings.a2)),
        "b2" => enigo.key_click(Key::Layout(settings.b2)),
        "y2" => enigo.key_click(Key::Layout(settings.y2)),

        // Server errors
        &_ => println!("Unkown input `{:?}`", input),
    }
}
