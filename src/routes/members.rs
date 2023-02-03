use crate::models::member::Member;
use crate::database::members::MemberData;
use crate::database::{members, AdventurersGuild};
use rocket::{
    serde::{json::Json, Deserialize},
    Route,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct RegisterResponse {
    member: Member,
}

#[derive(Serialize)]
pub struct IndexMembersResponse {
    members: Vec<Member>,
}

#[derive(Deserialize)]
pub struct AdventurerData {
    name: String,
}

#[post("/register", data = "<data>")]
pub async fn register(data: Json<AdventurerData>, db: AdventurersGuild) -> Json<RegisterResponse> {
    let member = db
        .run(move |conn| {
            members::create(
                &conn,
                MemberData {
                    username: &data.name[..],
                },
            )
        })
        .await;

    Json(RegisterResponse { member })
}

#[get("/")]
pub async fn index_members(db: AdventurersGuild) -> Json<IndexMembersResponse> {
    let members = db.run(|conn| members::index(&conn)).await;

    Json(IndexMembersResponse { members })
}

pub fn get_routes() -> Vec<Route> {
    routes![register, index_members]
}
