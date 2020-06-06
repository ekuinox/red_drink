use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use chrono::{NaiveDateTime};
use crate::schema::{users, users_roles, github_users};
use crate::models::role::Role;

/**
 * RedDrinkのユーザ
 */
#[table_name = "users"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Identifiable, Queryable, PartialEq, Clone, Copy, Debug)]
#[primary_key(id)]
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime
}

/**
 * Userの新規挿入用モデル
 */
#[table_name = "users"]
#[derive(Insertable, Debug)]
pub struct UserInsertable {
    pub id: Option<i32>
}

/**
 * GitHubアカウントとUserを紐付ける
 */
#[table_name = "github_users"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[primary_key(github_id, user_id)]
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
 * Userの所持権限
 */
#[table_name = "users_roles"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Role, foreign_key = "role_id")]
#[primary_key(role_id, user_id)]
pub struct UsersRole {
    pub role_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime
}

/**
 * 権限追加用
 */
#[table_name = "users_roles"]
#[derive(Insertable, Debug)]
pub struct UsersRoleInsertable {
    pub role_id: i32,
    pub user_id: i32
}

/** --- Implementations --- */

/**
 * 新規挿入用モデル
 */
impl UserInsertable {
    pub fn new() -> UserInsertable {
        UserInsertable { id: None }
    }
    pub fn new_with_id(id: i32) -> UserInsertable {
        UserInsertable { id: Some(id) }
    }
}

/**
 * ユーザの操作
 */
impl User {
    /**
     * 新規登録
     */
    pub fn create(user: UserInsertable, connection: &DBConnection) -> Option<User> {
        diesel::insert_into(users::table).values(&user).execute(connection)
            .map(|_| {
                users::table.order(users::id.desc()).first(connection).ok()
            })
            .ok()
            .flatten()
    }

    /**
     * GitHubアカウントと紐付ける
     */
    pub fn associate_to_github(self, github_user_id: i32, connection: &DBConnection) -> (User, bool) {
        (self, GitHubUserInsertable::new(self.id, github_user_id).create(connection))
    }

    /**
     * 権限を追加する
     */
    pub fn add_role(self, role_id: i32, connection: &DBConnection) -> (User, bool) {
        (self, UsersRoleInsertable::new(self.id, role_id).create(connection))
    }
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

impl UsersRoleInsertable {
    pub fn new(user_id: i32, role_id: i32) -> UsersRoleInsertable {
        UsersRoleInsertable { user_id: user_id, role_id: role_id }
    }
    /**
     * 新規追加
     */
    pub fn create(self, connection: &DBConnection) -> bool {
        diesel::insert_into(users_roles::table).values(self).execute(connection).is_ok()
    }
}