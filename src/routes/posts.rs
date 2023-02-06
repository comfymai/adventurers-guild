use crate::{
    database::posts::IndexingOptions,
    models::post::PostJson,
};

use rocket::{serde::json::Json, Route};
use serde::{Deserialize, Serialize};

use crate::database::{
    posts::{self, PostData},
    AdventurersGuild,
};

#[derive(Serialize)]
pub struct NewResponse {
    post: PostJson,
}

#[derive(Serialize)]
pub struct IndexResponse {
    posts: Vec<PostJson>,
}

#[derive(Deserialize)]
pub struct NewData {
    pub author_id: String,
    pub title: String,
    pub content: String,
    pub kind: i32,
}

#[post("/new", data = "<data>")]
pub async fn new(data: Json<NewData>, db: AdventurersGuild) -> Json<NewResponse> {
    let post = db
        .run(move |conn| {
            posts::create(
                &conn,
                PostData {
                    author_id: &data.author_id[..],
                    title: &data.title[..],
                    content: &data.content[..],
                    kind: data.kind,
                },
            )
        })
        .await;

    Json(NewResponse { post })
}

#[get("/", format = "json", data = "<filters>")]
pub async fn index(
    filters: Option<Json<IndexingOptions>>,
    db: AdventurersGuild,
) -> Json<IndexResponse> {
    let filters = match filters {
        Some(content) => content.into_inner(),
        None => IndexingOptions::default(),
    };

    let posts = db
        .run(move |conn| {
            posts::index(
                &conn,
                IndexingOptions {
                    id: filters.id,
                    author_id: filters.author_id,
                    title: filters.title,
                    kind: filters.kind,
                },
            )
        })
        .await;

    Json(IndexResponse { posts })
}

pub fn get_routes() -> Vec<Route> {
    routes![new, index]
}
