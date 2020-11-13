use chrono::NaiveDateTime;
use crate::schema::roles;

pub mod policy;
mod create_impl;
mod role_impl;

use policy::*;

#[table_name = "roles"]
#[derive(Identifiable, AsChangeset, Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug)]
#[primary_key(id)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub policy: Policy,
    pub created_at: NaiveDateTime
}
