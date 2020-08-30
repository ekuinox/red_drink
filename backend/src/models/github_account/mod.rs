use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use chrono::{NaiveDateTime};
use crate::schema::github_accounts;
use crate::models::user::User;

mod create_impl;

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

/**
 * GitHubアカウント挿入用
 */
#[table_name = "github_accounts"]
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
        diesel::insert_into(github_accounts::table).values(self).execute(connection).is_ok()
    }
}

impl GitHubAccount {
    /**
     * GitHubのユーザIDから取得する
     */
    pub fn find(github_id: i32, connection: &DBConnection) -> Option<GitHubAccount> {
        github_accounts::table.find(github_id).get_result::<GitHubAccount>(connection).ok()
    }

    /**
     * Userに変換する
     */
    pub fn to_user(self, connection: &DBConnection) -> Option<User> {
        User::find(self.user_id, connection)
    }
}
