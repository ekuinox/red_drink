use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;
use rocket::response::status::Custom;
use rocket::http::Status;
use chrono::NaiveDateTime;
use crate::db::Connection;
use crate::models::{User, Role, traits::Find};
use crate::auth::Claims;

#[derive(Serialize, Deserialize, Debug)]
pub struct Permission {
    pub path: String,
    pub resource_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleResponse {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub permissions: Vec<Permission>
}

#[get("/user/<username>/roles")]
pub fn get_roles(_claims: Claims, username: String, conn: Connection) -> Result<Json<Vec<RoleResponse>>, Custom<()>> {
    User::find(username, &conn)
        .and_then(|user| user.get_roles(&conn))
        .map(|roles| {
            roles.into_iter()
                .flat_map(|role| {
                    role.get_all_permissions(&conn).map(|permissions| (role, permissions))
                })
                .map(|(role, permissions): (Role, Vec<(String, String)>)| RoleResponse {
                    id: role.id,
                    name: role.name,
                    created_at: role.created_at,
                    permissions: permissions.into_iter()
                        .map(|(path, resource_id)| Permission { path, resource_id })
                        .collect::<Vec<Permission>>()
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
