use diesel;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::db::DBConnection;
use crate::models::{traits::*, Permission};
use crate::schema::permissions;
use crate::types::DieselError;

#[table_name = "permissions"]
#[derive(Serialize, Deserialize, Identifiable, Insertable, Queryable, Associations, PartialEq, Debug)]
#[primary_key(path)]
struct PermissionBuilder {
    path: String,
    name: String,
    description: Option<String>
}

impl PermissionBuilder {
    fn new(path: String, name: String) -> PermissionBuilder {
        PermissionBuilder {
            path, name, description: None
        }
    }
    fn description(self, description: String) -> PermissionBuilder {
        PermissionBuilder { description: Some(description), ..self }
    }
    fn save(self, conn: &DBConnection) -> Result<Permission, DieselError> {
        diesel::insert_into(permissions::table)
            .values(self)
            .returning((permissions::path, permissions::name, permissions::description, permissions::created_at))
            .get_result::<(String, String, Option<String>, NaiveDateTime)>(conn)
            .map(|(path, name, description, created_at)| Permission { path, name, description, created_at })
    }
}

impl Create<Permission, DieselError, (String, String)> for Permission {
    fn create((path, name): (String, String), conn: &DBConnection) -> Result<Permission, DieselError> {
        PermissionBuilder::new(path, name).save(conn)
    }
}

impl Create<Permission, DieselError, (String, String, String)> for Permission {
    fn create((path, name, description): (String, String, String), conn: &DBConnection) -> Result<Permission, DieselError> {
        PermissionBuilder::new(path, name).description(description).save(conn)
    }
}
