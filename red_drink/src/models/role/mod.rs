use chrono::NaiveDateTime;
use crate::schema::roles;

mod policy;
mod policies;
mod permission;
mod create_impl;
mod role_impl;

pub use policy::*;
pub use permission::*;
pub use policies::*;

#[table_name = "roles"]
#[derive(Identifiable, AsChangeset, Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug)]
#[primary_key(id)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub policies: Policies,
    pub created_at: NaiveDateTime
}
