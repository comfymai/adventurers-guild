use crate::db::{self, Adventurer, AdventurersGuild};
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize},
    Route,
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
    let adventurers = db.run(|conn| db::index(&conn)).await;

    Json(IndexAdventurersResponse { adventurers })
}

#[delete("/<id>")]
pub async fn delete_adventurer(id: String, db: AdventurersGuild) -> Status {
    let deleted_rows = db.run(|conn| db::delete(&conn, id)).await;

    if deleted_rows > 0 {
        Status::Ok
    } else {
        Status::NotFound
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![create_adventurer, index_adventurers, delete_adventurer]
}
