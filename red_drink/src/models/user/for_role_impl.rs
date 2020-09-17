use std::collections::HashSet;
use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use crate::types::DieselError;
use crate::models::{Assignment, User, Role, Permission, permission::HasPermission, traits::*};

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

    /// Userが持つPermissionを取得する
    pub fn get_permissions(&self, resource_id: Option<String>, connection: &DBConnection) -> Result<Vec<Permission>, DieselError> {
        self.get_roles(connection).map(
            |roles| roles.into_iter().flat_map(
                |role| role.get_permissions(resource_id.clone(), connection)
            ).flatten()
        ).map( // 重複の削除
            |permissions| permissions.into_iter()
                .collect::<HashSet<Permission>>()
                .into_iter()
                .collect::<Vec<Permission>>()
        )
    }

    /// Userが指定したリソースに対する権限を所有しているか
    pub fn has_permission(&self, required: String, resource_id: Option<String>, conn: &DBConnection) -> bool {
        self.get_permissions(resource_id, conn).map(
            |permissions| Permission::has_permission(&permissions, required)
        ).unwrap_or(false)
    }
}

#[test]
fn test_has_permission() {
    use crate::db::connect;
    let conn = connect().get().expect("cannnot get connection");
    conn.test_transaction::<_, diesel::result::Error, _>(|| {
        use crate::models::traits::*;
        use crate::models::Accessible;
        let user = User::create("name".to_string(), &conn).expect("cannot create user");
        let role = Role::create("test_role".to_string(), &conn).expect("cannot create role");
        if !user.add_role(role.id, &conn) {
            panic!("cannot attach role to user");
        }
        let paths = vec!["foo.bar".to_string(), "xxx.*".to_string()];
        paths.into_iter().for_each(|path| {
            use crate::models::permission::{Permission};
            let Permission { path, .. } = Permission::create((path.clone(), path.clone()), &conn).expect("cannot create role");
            // リソースは指定せず行う
            assert!(Accessible::create((role.id, path), &conn).is_ok());
        });

        // foo.barではもちろん*にはアクセスできない
        assert!(!user.has_permission("*".to_string(), None, &conn));
        // foo.*にもアクセスできない
        assert!(!user.has_permission("foo.*".to_string(), None, &conn));
        assert!(user.has_permission("foo.bar".to_string(), None, &conn));
        // エッbarより下にノードあるの!?という状況には対応しない
        assert!(!user.has_permission("foo.bar.baz".to_string(), None, &conn));
        assert!(!user.has_permission("foo.bar.*".to_string(), None, &conn));

        // xxx.*にアクセスできる
        assert!(user.has_permission("xxx.*".to_string(), None, &conn));
        // 子にもアクセスできる
        assert!(user.has_permission("xxx.yyy".to_string(), None, &conn));
        // 孫にもアクセスできる
        assert!(user.has_permission("xxx.yyy.zzz".to_string(), None, &conn));
        // xxx自体にはアクセスできない
        assert!(!user.has_permission("xxx".to_string(), None, &conn));

        Ok(())
    });
}
