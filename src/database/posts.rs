use crate::models::post::{Post, PostJson};
use crate::schema::posts;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum PostKind {
    Information,
    Discussion,
    Unknown,
}

impl From<i32> for PostKind {
    fn from(raw: i32) -> PostKind {
        match raw {
            0 => PostKind::Information,
            1 => PostKind::Discussion,
            _ => PostKind::Unknown,
        }
    }
}

impl From<PostKind> for i32 {
    fn from(kind: PostKind) -> i32 {
        match kind {
            PostKind::Information => 0,
            PostKind::Discussion => 1,
            PostKind::Unknown => -1,
        }
    }
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub id: &'a str,
    pub author_id: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub kind: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PostData<'a> {
    pub author_id: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub kind: i32,
}

pub fn create<'a>(conn: &PgConnection, data: PostData<'a>) -> PostJson {
    let new_post = NewPost {
        id: &Uuid::new_v4().to_string()[..],
        author_id: data.author_id,
        title: data.title,
        content: data.content,
        kind: data.kind,
    };

    diesel::insert_into(posts::table)
        .values(new_post)
        .get_result::<Post>(conn)
        .expect("failed to create post.")
        .to_json()
}

#[derive(Deserialize)]
pub struct IndexingOptions {
    pub id: Option<String>,
    pub author_id: Option<String>,
    pub title: Option<String>,
    pub kind: Option<i32>,
}

impl Default for IndexingOptions {
    fn default() -> IndexingOptions {
        IndexingOptions {
            id: None,
            author_id: None,
            title: None,
            kind: None,
        }
    }
}

pub fn index(conn: &PgConnection, options: IndexingOptions) -> Vec<PostJson> {
    let mut query = posts::table.into_boxed();

    if let Some(ref id) = options.id {
        query = query.filter(posts::id.eq(id))
    }
    if let Some(ref author_id) = options.author_id {
        query = query.filter(posts::author_id.eq(author_id))
    }
    if let Some(ref title) = options.title {
        query = query.filter(posts::title.eq(title))
    }
    if let Some(ref kind) = options.kind {
        query = query.filter(posts::kind.eq(kind))
    }

    query
        .load::<Post>(conn)
        .map(|result| result.into_iter().map(|post| post.to_json()).collect())
        .expect("failed to index posts.")
}
