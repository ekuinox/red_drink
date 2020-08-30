use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use chrono::{NaiveDateTime};
use crate::schema::assignments;
use crate::models::user::*;
use crate::models::role::Role;

/**
 * Userの所持権限
 */
#[table_name = "assignments"]
#[derive(Identifiable, AsChangeset, Serialize, Deserialize, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Role, foreign_key = "role_id")]
#[primary_key(user_id, role_id)]
pub struct UsersRole {
    pub user_id: i32,
    pub role_id: i32,
    pub created_at: NaiveDateTime
}

/**
 * 権限追加用
 */
#[table_name = "assignments"]
#[derive(Insertable, Debug)]
pub struct UsersRoleInsertable {
    pub user_id: i32,
    pub role_id: i32
}

impl UsersRoleInsertable {
    pub fn new(user_id: i32, role_id: i32) -> UsersRoleInsertable {
        UsersRoleInsertable { user_id: user_id, role_id: role_id }
    }
    /**
     * 新規追加
     */
    pub fn create(self, connection: &DBConnection) -> bool {
        connection.transaction(|| {
            diesel::insert_into(assignments::table).values(self).execute(connection)
        }).is_ok()
    }
}