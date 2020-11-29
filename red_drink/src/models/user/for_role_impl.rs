use diesel;
use diesel::prelude::*;
use crate::{db::DBConnection, models::resource_id::ResourceId};
use crate::types::DieselError;
use crate::models::{Assignment, User, Role, traits::*};

/// Userに対してのRole周辺の実装
impl User {
    
    /// ユーザにRoleを付与する
    pub fn add_role(&self, role_id: i32, connection: &DBConnection) -> bool {
        Assignment::create((self.id, role_id), connection).is_ok()
    }

    /// ユーザの持つRoleを取得する
    pub fn get_roles(&self, connection: &DBConnection) -> Result<Vec<Role>, DieselError> {
        Assignment::belonging_to(self).get_results::<Assignment>(connection).map(|users_roles| {
            users_roles.iter().flat_map(|users_role| {
                Role::find(users_role.role_id, connection)
            }).collect::<Vec<Role>>()
        })
    }

    /// Userが指定したリソースに対する権限を所有しているか
    pub fn has_permission(&self, required: String, resource_id: ResourceId, conn: &DBConnection) -> bool {
        let roles = self.get_roles(conn).unwrap_or(vec![]);
        roles.into_iter().any(|role| role.has_permission(required.clone(), resource_id.clone()));
        false
    }
}

#[test]
fn test_has_permission() {
    // TODO
}
