use chrono::{NaiveDateTime};
use crate::schema::assignments;
use crate::models::{User, Role};

mod create_impl;

/**
 * Userの所持権限
 */
#[table_name = "assignments"]
#[derive(Identifiable, AsChangeset, Serialize, Deserialize, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Role, foreign_key = "role_id")]
#[primary_key(user_id, role_id)]
pub struct Assignment {
    pub user_id: i32,
    pub role_id: i32,
    pub created_at: NaiveDateTime
}

