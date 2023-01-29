use crate::AppState;
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize},
    State,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct Adventurer {
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct IndexAdventurersResponse {
    adventurers: Vec<Adventurer>,
}

#[derive(Deserialize)]
pub struct AdventurerData {
    name: String,
}

#[post("/adventurer", data = "<data>")]
pub fn create_adventurer(data: Json<AdventurerData>, state: &State<AppState>) -> Status {
    let mut adventurers = state.adventurers.lock().unwrap();

    adventurers.push(Adventurer {
        id: Uuid::new_v4().to_string(),
        name: data.name.clone(),
    });

    Status::Created
}

#[get("/adventurer")]
pub fn index_adventurers(state: &State<AppState>) -> Json<IndexAdventurersResponse> {
    let adventurers = state.adventurers.lock().unwrap();

    Json(IndexAdventurersResponse {
        adventurers: adventurers.to_vec(),
    })
}

#[delete("/adventurer/<id>")]
pub fn delete_adventurer(id: String, state: &State<AppState>) -> Status {
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
