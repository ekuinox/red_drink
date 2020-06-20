use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use chrono::NaiveDateTime;
use crate::schema::{permissions};

#[table_name = "permissions"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug)]
#[primary_key(path)]
pub struct Permission {
    path: String,
    name: String,
    description: Option<String>,
    created_at: NaiveDateTime,
}

impl Permission {
    pub fn find(path: String, connection: &DBConnection) -> Option<Permission> {
        permissions::table.find(path).get_result::<Permission>(connection).ok()
    }
}

#[table_name = "permissions"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug, Clone)]
#[primary_key(path)]
pub struct PermissionInsertable {
    path: String,
    name: String,
    description: Option<String>
}

impl PermissionInsertable {
    pub fn new(path: String, name: String, description: Option<String>) -> PermissionInsertable {
        PermissionInsertable {
            path, name, description
        }
    }
    
    pub fn create(&self, connection: &DBConnection) -> Option<Permission> {
        let _ = diesel::insert_into(permissions::table).values((*self).clone()).execute(connection);
        Permission::find(self.path.clone(), connection)
    }
}