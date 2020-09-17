use chrono::{NaiveDateTime};
use crate::schema::users;

mod as_user;
mod create_impl;
mod user_impl;
mod for_github_impl;
mod for_role_impl;

/// RedDrinkのユーザ
#[table_name = "users"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Identifiable, Associations, Queryable, PartialEq, Clone, Debug)]
#[primary_key(id)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub created_at: NaiveDateTime
}

pub use as_user::AsUser;
pub use for_github_impl::GitHubAccountDetail;
