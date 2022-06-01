mod anthem;

use crate::anthem::AnthemIP;
use rocket::State;
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Anthem IP Controller"
}

#[get("/avr_on")]
fn avr_on(avr: &State<Arc<Mutex<AnthemIP>>>) -> &'static str {
    let _ = avr.lock().unwrap().set_power(true);
    "powered on"
}

#[get("/avr_off")]
fn avr_off(avr: &State<Arc<Mutex<AnthemIP>>>) -> &'static str {
    let _ = avr.lock().unwrap().set_power(false);
    "powered off"
}

#[get("/avr_set_input/<input_num>")]
fn avr_set_input(avr: &State<Arc<Mutex<AnthemIP>>>, input_num: u8) -> &'static str {
    let _ = avr.lock().unwrap().set_current_input(input_num);
    "set current input"
}

#[launch]
fn rocket() -> _ {
    let mut anthem = AnthemIP::new("192.168.0.28", "14999");
    rocket::build()
        .manage(Arc::new(Mutex::new(anthem)))
        .mount("/", routes![index])
        .mount("/", routes![avr_on])
        .mount("/", routes![avr_off])
        .mount("/", routes![avr_set_input])
}
