use chrono::NaiveDateTime;
use crate::schema::accessibles;
use crate::models::{Resource, Role, Permission};

#[table_name = "accessibles"]
#[derive(Identifiable, AsChangeset, Serialize, Deserialize, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Role, foreign_key = "role_id")]
#[belongs_to(Permission, foreign_key = "permission_path")]
#[belongs_to(Resource, foreign_key = "resource_id")]
#[primary_key(role_id, permission_path, resource_id)]
pub struct Accessible {
    pub role_id: i32,
    pub permission_path: String,
    pub resource_id: String,
    pub created_at: NaiveDateTime
}
