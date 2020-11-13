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
        roles.into_iter().all(|role| role.has_permission(required.clone(), resource_id.clone()));
        false
    }
}

#[test]
fn test_has_permission() {
    use crate::db::connect;
    let conn = connect().get().expect("cannnot get connection");
    conn.test_transaction::<_, diesel::result::Error, _>(|| {
        use crate::models::traits::*;
        use crate::models::{role::policy::{Policy, Allowed}, resource_id::ROOT_RESOURCE};
        let user = User::create("name".to_string(), &conn).expect("cannot create user");
        let allowed = Allowed {
            resources: vec![ROOT_RESOURCE.clone()],
            permissions: vec!["foo.bar".to_string(), "xxx.*".to_string()]
        };
        let policy = Policy::with_allowed(allowed);
        let role = Role::create(("test_role".to_string(), policy) , &conn).expect("cannot create role");
        if !user.add_role(role.id, &conn) {
            panic!("cannot attach role to user");
        }
        // foo.barではもちろん*にはアクセスできない
        assert!(!user.has_permission("*".to_string(), ROOT_RESOURCE.clone(), &conn));
        // foo.*にもアクセスできない
        assert!(!user.has_permission("foo.*".to_string(), ROOT_RESOURCE.clone(), &conn));
        assert!(user.has_permission("foo.bar".to_string(), ROOT_RESOURCE.clone(), &conn));
        // エッbarより下にノードあるの!?という状況には対応しない
        assert!(!user.has_permission("foo.bar.baz".to_string(), ROOT_RESOURCE.clone(), &conn));
        assert!(!user.has_permission("foo.bar.*".to_string(), ROOT_RESOURCE.clone(), &conn));

        // xxx.*にアクセスできる
        assert!(user.has_permission("xxx.*".to_string(), ROOT_RESOURCE.clone(), &conn));
        // 子にもアクセスできる
        assert!(user.has_permission("xxx.yyy".to_string(), ROOT_RESOURCE.clone(), &conn));
        // 孫にもアクセスできる
        assert!(user.has_permission("xxx.yyy.zzz".to_string(), ROOT_RESOURCE.clone(), &conn));
        // xxx自体にはアクセスできない
        assert!(!user.has_permission("xxx".to_string(), ROOT_RESOURCE.clone(), &conn));

        Ok(())
    });
}
