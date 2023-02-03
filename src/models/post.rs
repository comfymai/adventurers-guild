use serde::Serialize;

#[derive(Queryable, Serialize, Clone)]
pub struct Post {
    pub id: String,
    pub author_id: String,
    pub title: String,
    pub content: String,
    pub kind: i32,
}

impl Post {
    pub fn to_json(self) -> PostJson {
        PostJson {
            id: self.id,
            author_id: self.author_id,
            title: self.title,
            content: self.content,
            kind: self.kind
        }
    }
}

#[derive(Serialize)]
pub struct PostJson {
    pub id: String,
    pub author_id: String,
    pub title: String,
    pub content: String,
    pub kind: i32,
}
