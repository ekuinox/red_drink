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
        use super::Includes;
        self.policies
            .iter()
            .filter(|policy| policy.is_allowed()) // TODO: Accessibleがどうかはとりあえず見ない
            .any(|policy| policy.includes((permission.clone(), resource.clone())))
    }
    // get all roles
    pub fn all(connection: &DBConnection) -> Result<Vec<Role>, DieselError> {
        roles::table.load::<Role>(connection)
    }
}

#[test]
fn test_has_permission() {
    use chrono::Utc;
    use crate::models::resource_id::ROOT_RESOURCE;
    use super::{Policy, Policies, Permission};

    let p1 = Policy {
        resources: vec![ROOT_RESOURCE.clone()],
        permissions: vec![Permission::from("foo.bar.*"), Permission::from("xxx.*")],
        ..Default::default()
    };
    
    let r1 = Role {
        id: 0,
        name: "test role".to_string(),
        policies: Policies::from(p1),
        created_at: Utc::now().naive_utc()
    };

    assert!(r1.has_permission("foo.bar.baz".to_string(), ROOT_RESOURCE.clone()));
    assert!(r1.has_permission("foo.bar.*".to_string(), ROOT_RESOURCE.clone()));
    assert!(r1.has_permission("xxx.yyy.zzz".to_string(), ROOT_RESOURCE.clone()));
    assert!(!r1.has_permission("foo.abc.*".to_string(), ROOT_RESOURCE.clone()));
    assert!(!r1.has_permission("abc.*".to_string(), ROOT_RESOURCE.clone()));
}
