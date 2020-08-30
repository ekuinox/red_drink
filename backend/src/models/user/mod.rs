use chrono::{NaiveDateTime};
use crate::schema::users;

mod create_impl;
mod user_impl;
mod for_github_impl;
mod for_role_impl;

/// RedDrinkのユーザ
#[table_name = "users"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Identifiable, Associations, Queryable, PartialEq, Clone, Copy, Debug)]
#[primary_key(id)]
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime
}
