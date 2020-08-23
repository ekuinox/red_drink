use std::collections::HashSet;
use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use crate::models::users_roles::*;
use crate::models::role::Role;
use crate::models::permission::Permission;
use crate::models::permission::HasPermission;
use crate::models::user::User;

/// Userに対してのRole周辺の実装
impl User {
    
    /// ユーザにRoleを付与する
    pub fn add_role(&self, role_id: i32, connection: &DBConnection) -> bool {
        UsersRoleInsertable::new(self.id, role_id).create(connection)
    }

    /// ユーザの持つRoleを取得する
    pub fn get_roles(&self, connection: &DBConnection) -> Option<Vec<Role>> {
        UsersRole::belonging_to(self).get_results::<UsersRole>(connection).map(|users_roles| {
            users_roles.iter().flat_map(|users_role| {
                Role::find(users_role.role_id, connection)
            }).collect::<Vec<Role>>()
        }).ok()
    }

    /// Userが持つPermissionを取得する
    pub fn get_permissions(&self, connection: &DBConnection) -> Option<Vec<Permission>> {
        self.get_roles(connection).map(
            |roles| roles.into_iter().flat_map(
                |role| role.get_permissions(connection)
            ).collect::<Vec<Vec<Permission>>>().concat()
        ).map( // 重複の削除
            |permissions| permissions.into_iter()
                .collect::<HashSet<Permission>>()
                .into_iter()
                .collect::<Vec<Permission>>()
        )
    }

    /// Userが指定した権限を所有しているか
    pub fn has_permission(&self, required: String, conn: &DBConnection) -> bool {
        self.get_permissions(conn).map(
            |permissions| Permission::has_permission(&permissions, required)
        ).unwrap_or(false)
    }
}
