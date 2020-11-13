use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;
use rocket::response::status::Custom;
use rocket::http::Status;
use chrono::NaiveDateTime;
use crate::db::Connection;
use crate::models::{User, traits::Find, resource_id::ResourceId};
use crate::auth::Claims;

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    pub resource_id: ResourceId,
    pub permissions: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleResponse {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub resources: Vec<Resource>
}

#[get("/user/<username>/roles")]
pub fn get_roles(_claims: Claims, username: String, conn: Connection) -> Result<Json<Vec<RoleResponse>>, Custom<()>> {
    User::find(username, &conn)
        .and_then(|user| user.get_roles(&conn))
        .map(|roles| {
            roles.into_iter()
                .map(|role| RoleResponse {
                    id: role.id,
                    name: role.name,
                    created_at: role.created_at,
                    resources: vec![] // TODO: impl
                })
                .collect::<Vec<RoleResponse>>()
        })
        .map_err(|err| Custom(match err {
            red_drink::types::DieselError::NotFound => Status::NotFound,
            _ => Status::InternalServerError
        }, ())
        )
        .map(|roles| Json(roles))
}
