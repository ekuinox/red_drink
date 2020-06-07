use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use chrono::{NaiveDateTime};
use crate::schema::users;
use crate::models::github_user::*;
use crate::models::users_roles::*;

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
     * GitHubアカウントから作成する
     * 注意: GitHubUserへの挿入が失敗した場合でも、そのレコードの削除を行わなっていない
     */
    pub fn create_with_github_id(github_id: i32, connection: &DBConnection) -> Option<User> {
        Self::create(UserInsertable::new(), connection).and_then(|user| {
            match user.associate_to_github(github_id, connection) {
                (user, true) => Some(user),
                _ => None
            }
        })
    }

    /**
     * GitHubアカウントと紐付ける
     */
    pub fn associate_to_github(self, github_user_id: i32, connection: &DBConnection) -> (User, bool) {
        (self, GitHubUserInsertable::new(self.id, github_user_id).create(connection))
    }

    /**
     * ユーザにRoleを付与する
     */
    pub fn add_role(&self, role_id: i32, connection: &DBConnection) -> bool {
        UsersRoleInsertable::new(self.id, role_id).create(connection)
    }

    /**
     * ユーザIDから取得する
     */
    pub fn find(id: i32, connection: &DBConnection) -> Option<User> {
        users::table.find(id).get_result(connection).ok()
    }

    /**
     * GitHubIdから検索する
     */
    pub fn find_by_github_id(github_id: i32, connection: &DBConnection) -> Option<User> {
        GitHubUser::find(github_id, connection).and_then(|user: GitHubUser| {
            user.to_user(connection)
        })
    }

    /**
     * GitHubIdから検索し、存在しなければ作成する
     */
    pub fn find_or_new_by_github_id(github_id: i32, connection: &DBConnection) -> Option<User> {
        Self::find_by_github_id(github_id, connection).or_else(|| {
            User::create_with_github_id(github_id, &connection)
        })
    }
}