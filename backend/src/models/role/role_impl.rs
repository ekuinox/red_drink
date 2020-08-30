use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use crate::schema::{roles, assignments};
use crate::types::DieselError;
use crate::models::{Role, Permission, Accessible, traits::*};

impl Find<Role, DieselError, i32> for Role {
    fn find(id: i32, conn: &DBConnection) -> Result<Role, DieselError> {
        roles::table.find(id).first::<Role>(conn)
    }
}

impl Role {
    /**
     * Userに紐づくRoleを取得する
     */
    pub fn get_roles(user_id: i32, connection: &DBConnection) -> Result<Vec<Role>, DieselError> {
        assignments::table.inner_join(roles::table)
            .filter(assignments::user_id.eq(user_id))
            .select((roles::id, roles::name, roles::created_at))
            .load::<Role>(connection)
    }

    /**
     * Roleに紐づくPermissionを取得する
     */
    pub fn get_permissions(&self, resource_id: Option<String>, conn: &DBConnection) -> Result<Vec<Permission>, DieselError> {
        match resource_id {
            Some(id) => Accessible::get_permissions(self.id, id, conn),
            None => Accessible::get_permissions_for_root(self.id, conn)
        }
    }

    /**
     * Roleにリソースに対してのPermissionを紐付ける
     */
    pub fn attach_permission(&self, permission_path: String, resource_id: Option<String>, conn: &DBConnection) -> Result<Accessible, diesel::result::Error> {
        if let Some(resource_id) = resource_id {
            Accessible::create((self.id, permission_path, resource_id), conn)
        } else {
            Accessible::create((self.id, permission_path), conn)
        }
    }

    // get all roles
    pub fn all(connection: &DBConnection) -> Result<Vec<Role>, DieselError> {
        roles::table.load::<Role>(connection)
    }
}
