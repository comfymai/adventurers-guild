use crate::database::members::{IndexingOptions, MemberData};
use crate::database::{members, AdventurersGuild};
use crate::models::member::MemberJson;
use rocket::{
    serde::{json::Json, Deserialize},
    Route,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct RegisterResponse {
    member: MemberJson,
}

#[derive(Serialize)]
pub struct IndexMembersResponse {
    members: Vec<MemberJson>,
}

#[derive(Deserialize)]
pub struct IndexData {
    id: Option<String>,
    username: Option<String>,
}

impl Default for IndexData {
    fn default() -> Self {
        Self {
            id: None,
            username: None
        }
    }
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

#[get("/", format="json", data = "<filters>")]
pub async fn index_members(
    filters: Option<Json<IndexData>>,
    db: AdventurersGuild,
) -> Json<IndexMembersResponse> {
    let filters = match filters {
        Some(content) => content.into_inner(),
        None => IndexData::default()
    };

    let members = db
        .run(move |conn| {
            members::index(
                &conn,
                IndexingOptions {
                    id: filters.id,
                    username: filters.username,
                },
            )
        })
        .await;

    Json(IndexMembersResponse { members })
}

pub fn get_routes() -> Vec<Route> {
    routes![register, index_members]
}
