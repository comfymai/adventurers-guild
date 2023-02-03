use serde::Serialize;

#[derive(Queryable, Serialize, Clone)]
pub struct Member {
    pub id: String,
    pub username: String,
}

impl Member {
    pub fn to_json(self) -> MemberJson {
        MemberJson {
            id: self.id,
            username: self.username,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberJson {
    pub id: String,
    pub username: String,
}
