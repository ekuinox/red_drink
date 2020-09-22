use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;
use rocket::response::status::{Custom, NotFound};
use rocket::http::Status;
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
pub fn get(claims: Claims, conn: Connection) -> Result<Json<UserResponse>, Custom<()>> {
    use crate::models::{User, user::AsUser};
    claims.as_user(&conn)
        .map(|User { id, name, avatar_url, email, created_at }: User| UserResponse {
            id, name, avatar_url, email, created_at
        })
        .map_err(|_| Custom(Status::InternalServerError, ()))
        .map(|user| Json(user))
}

#[get("/user/<username>")]
pub fn get_user_by_username(_claims: Claims, username: String, conn: Connection) -> Result<Json<UserResponse>, NotFound<()>> {
    use crate::models::{User, traits::Find};
    User::find(username, &conn)
        .map(|User { id, name, avatar_url, email, created_at }: User| UserResponse {
            id, name, avatar_url, email, created_at
        })
        .map_err(|_| NotFound(()))
        .map(|user| Json(user))
}
