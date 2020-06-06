use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use chrono::{NaiveDateTime};
use crate::schema::github_users;
use crate::models::user::User;

/**
 * GitHubアカウントとUserを紐付ける
 */
#[table_name = "github_users"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[primary_key(user_id)]
pub struct GitHubUser {
    pub github_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime
}

/**
 * GitHubアカウント挿入用
 */
#[table_name = "github_users"]
#[derive(Insertable, Debug)]
pub struct GitHubUserInsertable {
    pub github_id: i32,
    pub user_id: i32
}

/**
 * GitHub挿入用
 */
impl GitHubUserInsertable {
    pub fn new(user_id: i32, github_id: i32) -> GitHubUserInsertable {
        GitHubUserInsertable { github_id: github_id, user_id: user_id }
    }
    /**
     * 新規追加
     */
    pub fn create(self, connection: &DBConnection) -> bool {
        diesel::insert_into(github_users::table).values(self).execute(connection).is_ok()
    }
}

impl GitHubUser {
    /**
     * GitHubのユーザIDから取得する
     */
    pub fn find(github_id: i32, connection: &DBConnection) -> Option<GitHubUser> {
        github_users::table.filter(github_users::github_id.eq(github_id)).first::<GitHubUser>(connection).ok()
    }
}