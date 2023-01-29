use rocket::{
    http::Status,
    serde::{json::Json, Deserialize},
    State,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Clone)]
struct Adventurer {
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
struct IndexAdventurersResponse {
    adventurers: Vec<Adventurer>,
}

struct AppState {
    adventurers: Arc<Mutex<Vec<Adventurer>>>,
}

#[derive(Deserialize)]
struct AdventurerData {
    name: String,
}

#[post("/adventurer", data = "<data>")]
fn create_adventurer(data: Json<AdventurerData>, state: &State<AppState>) -> Status {
    let mut adventurers = state.adventurers.lock().unwrap();

    adventurers.push(Adventurer {
        id: Uuid::new_v4().to_string(),
        name: data.name.clone(),
    });

    Status::Created
}

#[get("/adventurer")]
fn index_adventurers(state: &State<AppState>) -> Json<IndexAdventurersResponse> {
    let adventurers = state.adventurers.lock().unwrap();

    Json(IndexAdventurersResponse {
        adventurers: adventurers.to_vec(),
    })
}

#[delete("/adventurer/<id>")]
fn delete_adventurer(id: String, state: &State<AppState>) -> Status {
    let mut adventurers = state.adventurers.lock().unwrap();

    match adventurers
        .iter()
        .position(|adventurer| adventurer.id == id)
    {
        Some(index) => {
            adventurers.remove(index);
            Status::Ok
        }
        None => Status::NotFound,
    }
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
