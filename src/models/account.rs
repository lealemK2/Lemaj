use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Account {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
}
