mod controllers;

use controllers::adventurer::{
    create_adventurer, delete_adventurer, index_adventurers, Adventurer,
};
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate rocket;

pub struct AppState {
    adventurers: Arc<Mutex<Vec<Adventurer>>>,
}

#[launch]
fn app() -> _ {
    rocket::build()
        .manage(AppState {
            adventurers: Arc::new(Mutex::new(vec![])),
        })
        .mount(
            "/",
            routes![create_adventurer, index_adventurers, delete_adventurer],
        )
}
