use chrono::NaiveDateTime;
use crate::schema::permissions;

mod create_impl;
mod has_permission_impl;
mod permission_impl;

#[table_name = "permissions"]
#[derive(AsChangeset, Serialize, Deserialize, Identifiable, Insertable, Queryable, PartialEq, Eq, Hash, Clone, Debug)]
#[primary_key(path)]
pub struct Permission {
    pub path: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

pub(crate) trait HasPermission<T, R> {
    /**
     * Permission配列に欲しいPermissionが含まれているか
     */
    fn has_permission(permissions: T, required: R) -> bool;
}
