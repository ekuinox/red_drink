use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;
use chrono::NaiveDateTime;
use crate::db::Connection;
use crate::auth::Claims;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserResponse {
    id: i32,
    name: String,
    avatar_url: Option<String>,
    email: Option<String>,
    created_at: NaiveDateTime
}

#[get("/user")]
pub fn get(claims: Claims, conn: Connection) -> Json<Option<UserResponse>> {
    use crate::models::{User, user::AsUser};
    Json(claims.as_user(&conn)
        .map(|User { id, name, avatar_url, email, created_at }: User| UserResponse {
            id, name, avatar_url, email, created_at
        })
        .ok()
    )
}
