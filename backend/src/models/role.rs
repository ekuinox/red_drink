use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use chrono::{Utc, NaiveDateTime};
use crate::schema::{roles, roles_permissions};
use crate::models::permission::Permission;

static ADMIN_ROLE_ID: i32 = 0;

#[table_name = "roles"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug)]
#[primary_key(id)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime
}

#[table_name = "roles_permissions"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Role, foreign_key = "role_id")]
#[belongs_to(Permission, foreign_key = "permission_path")]
#[primary_key(role_id, permission_path)]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_path: String,
    pub created_at: NaiveDateTime
}
