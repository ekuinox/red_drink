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
        roles.into_iter().any(|role| role.has_permission(required.clone(), resource_id.clone()))
    }
}

#[test]
fn test_has_permission() {
    use crate::models::resource_id::ROOT_RESOURCE;
    use crate::models::role::{Policy, Permission, Role};
    use crate::db::connect;

    let p1 = Policy {
        resources: vec![ROOT_RESOURCE.clone()],
        permissions: vec![Permission::from("foo.bar.*"), Permission::from("xxx.*")],
        ..Default::default()
    };
    
    let conn = connect().get().expect("cannnot get connection");
    conn.test_transaction::<_, diesel::result::Error, _>(|| {
        let u1 = User::create("test user".to_string(), &conn)?;
        let r1 = Role::create(("test role".to_string(), p1), &conn)?;
        u1.add_role(r1.id, &conn);

        assert!(u1.has_permission("foo.bar.baz".to_string(), ROOT_RESOURCE.clone(), &conn));
        assert!(u1.has_permission("foo.bar.*".to_string(), ROOT_RESOURCE.clone(), &conn));
        assert!(u1.has_permission("xxx.yyy.zzz".to_string(), ROOT_RESOURCE.clone(), &conn));
        assert!(!u1.has_permission("foo.abc.*".to_string(), ROOT_RESOURCE.clone(), &conn));
        assert!(!u1.has_permission("abc.*".to_string(), ROOT_RESOURCE.clone(), &conn));

        Ok(())
    });
}
