use chrono::NaiveDateTime;
use crate::schema::roles;

pub mod accessible;
mod create_impl;
mod role_impl;

use accessible::*;

#[table_name = "roles"]
#[derive(Identifiable, AsChangeset, Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug)]
#[primary_key(id)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub accessible: Accessible,
    pub created_at: NaiveDateTime
}
