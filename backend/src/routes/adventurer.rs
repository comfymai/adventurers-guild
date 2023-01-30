use crate::{
    db::{self, Adventurer, AdventurersGuild},
    AppState,
};
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize},
    Route, State,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct IndexAdventurersResponse {
    adventurers: Vec<Adventurer>,
}

#[derive(Deserialize)]
pub struct AdventurerData {
    name: String,
}

#[post("/", data = "<data>")]
pub async fn create_adventurer(data: Json<AdventurerData>, db: AdventurersGuild) -> Status {
    db.run(move |conn| {
        db::create(&conn, data.name.to_string());
    })
    .await;

    Status::Created
}

#[get("/")]
pub async fn index_adventurers(db: AdventurersGuild) -> Json<IndexAdventurersResponse> {
    let adventurers = db.run(|conn| {
        db::index(&conn)
    }).await;

    Json(IndexAdventurersResponse {
        adventurers,
    })
}

#[delete("/<id>")]
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

pub fn get_routes() -> Vec<Route> {
    routes![create_adventurer, index_adventurers, delete_adventurer]
}
