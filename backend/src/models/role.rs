use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use chrono::{Utc, NaiveDateTime};
use crate::schema::{roles, roles_permissions};
use crate::models::permission::Permission;

pub const ADMIN_ROLE_ID: i32 = 0;

#[table_name = "roles"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug)]
#[primary_key(id)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime
}

impl Role {
    /**
     * Roleを作成する
     */
    pub fn new(id: i32, name: String, created_at: NaiveDateTime) -> Role {
        Role {
            id: id,
            name: name,
            created_at: created_at
        }
    }

    /**
     * tupleからRoleを作成する
     */
    pub fn new_from_tuple(t: (i32, String, NaiveDateTime)) -> Role {
        Self::new(t.0, t.1, t.2)
    }
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
