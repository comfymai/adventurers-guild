use crate::database::members::{IndexingOptions, MemberCreationError, MemberData};
use crate::database::{members, AdventurersGuild};
use crate::models::member::MemberJson;
use rocket::{
    serde::{json::Json, Deserialize},
    Route,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    member: MemberJson,
}

#[derive(Deserialize)]
pub struct RegisterData {
    name: String,
}

#[post("/register", format = "json", data = "<data>")]
pub async fn register(
    data: Json<RegisterData>,
    db: AdventurersGuild,
) -> Result<Json<RegisterResponse>, Json<ErrorResponse>> {
    let result = db
        .run(move |conn| {
            members::create(
                &conn,
                MemberData {
                    username: data.name.clone(),
                },
            )
        })
        .await;

    match result {
        Ok(member) => Ok(Json(RegisterResponse { member })),
        Err(error) => match error {
            MemberCreationError::DuplicateUsername => Err(Json(ErrorResponse {
                message: "There's already a member registered with the same username.".to_string(),
            })),
            MemberCreationError::Generic => Err(Json(ErrorResponse {
                message: "Something went wrong when trying to register member.".to_string(),
            })),
        },
    }
}

#[derive(Serialize)]
pub struct IndexResponse {
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
            username: None,
        }
    }
}

#[get("/", format = "json", data = "<filters>")]
pub async fn index_members(
    filters: Option<Json<IndexData>>,
    db: AdventurersGuild,
) -> Json<IndexResponse> {
    let filters = match filters {
        Some(content) => content.into_inner(),
        None => IndexData::default(),
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

    Json(IndexResponse { members })
}

pub fn get_routes() -> Vec<Route> {
    routes![register, index_members]
}
