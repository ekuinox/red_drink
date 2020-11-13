use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use crate::schema::roles;
use crate::types::DieselError;
use crate::models::{Role, traits::*};
use crate::models::resource_id::ResourceId;

impl Find<Role, DieselError, i32> for Role {
    fn find(id: i32, conn: &DBConnection) -> Result<Role, DieselError> {
        roles::table.find(id).first::<Role>(conn)
    }
}

impl Role {
    /// リソースに対する権限があるか取得する
    pub fn has_permission(&self, permission: String, resource: ResourceId) -> bool {
        self.policy.is_allowed(resource, permission)
    }
    // get all roles
    pub fn all(connection: &DBConnection) -> Result<Vec<Role>, DieselError> {
        roles::table.load::<Role>(connection)
    }
}
