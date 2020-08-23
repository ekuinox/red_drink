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
        self.get_roles(connection).map(|roles| {
            roles.into_iter().fold(Vec::<Permission>::new(), |prev, role| {
                role.get_permissions(connection).map(|permissions| [&prev[..], &permissions[..]].concat()).unwrap_or(prev)
            })
        }).map(|permissions| {
            // 重複を取り除く
            permissions.into_iter().collect::<HashSet<Permission>>().into_iter().collect::<Vec<Permission>>()
        })
    }

    /// Userが指定した権限を所有しているか
    pub fn has_permission(&self, required: String, connection: &DBConnection) -> bool {
        self.get_permissions(connection).map(|permissions| Permission::has_permission(&permissions, required)).is_some()
    }
}
