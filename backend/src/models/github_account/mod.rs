use chrono::{NaiveDateTime};
use crate::schema::github_accounts;
use crate::models::user::User;

mod create_impl;
mod github_account_impl;

/**
 * GitHubアカウントとUserを紐付ける
 */
#[table_name = "github_accounts"]
#[derive(AsChangeset, Serialize, Deserialize, Identifiable, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[primary_key(github_id)]
pub struct GitHubAccount {
    pub github_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime
}
