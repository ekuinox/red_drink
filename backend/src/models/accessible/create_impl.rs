use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use diesel::result::Error as DieselError;
use crate::db::DBConnection;
use crate::models::{traits::*, Accessible};
use crate::schema::accessibles;

#[table_name = "accessibles"]
#[derive(Serialize, Deserialize, Identifiable, Insertable, PartialEq, Debug, Default)]
#[primary_key(role_id, permission_path, resource_id)]
struct AccessibleBuilder {
    role_id: i32,
    permission_path: String,
    resource_id: Option<String>
}

impl AccessibleBuilder {
    fn new(role: i32, permission: String) -> AccessibleBuilder {
        AccessibleBuilder {
            role_id: role,
            permission_path: permission,
            ..AccessibleBuilder::default()
        }
    }
    fn resource_id(self, resource: String) -> AccessibleBuilder {
        AccessibleBuilder {
            resource_id: Some(resource),
            ..self
        }
    }
    fn save(self, conn: &DBConnection) -> Result<Accessible, DieselError> {
        // if self.resuorce_id is None, insert * as resource_id
        let (role_id, permission_path, resource_id) = diesel::insert_into(Self::table())
            .values(self)
            .returning((accessibles::role_id, accessibles::permission_path, accessibles::resource_id))
            .get_result::<(i32, String, String)>(conn)?;
        Self::table().find((role_id, permission_path, resource_id)).first::<Accessible>(conn)
    }
}

// attach permission to role with root resource
impl Create<Accessible, DieselError, (i32, String)> for Accessible {
    fn create((role_id, permission_path): (i32, String), conn: &DBConnection) -> Result<Accessible, DieselError> {
        AccessibleBuilder::new(role_id, permission_path).save(conn)
    }
}

// attach permission to role with specify resource
impl Create<Accessible, DieselError, (i32, String, String)> for Accessible {
    fn create((role_id, permission_path, resource_id): (i32, String, String), conn: &DBConnection) -> Result<Accessible, DieselError> {
        AccessibleBuilder::new(role_id, permission_path).resource_id(resource_id).save(conn)
    }
}

#[test]
fn test_create_accessible() {
    use crate::db::connect;
    let conn = connect().get().expect("could not connect db");
    conn.test_transaction::<_, diesel::result::Error, _>(|| {
        use crate::models::*;
        let permission: Permission = PermissionInsertable::new("foo.*".to_string(), "foo root".to_string(), None).create(&conn).expect("could not create permission");
        let role: Role = RoleInsertable::new("foo role".to_string()).create(&conn).expect("could not create role");

        // root
        let accessible = Accessible::create((role.id, permission.path.clone()), &conn)?;
        assert_eq!(accessible.resource_id, "*".to_string());

        // リソース指定
        let resource: Resource = Resource::create("foo resource".to_string(), &conn)?;
        let accessible = Accessible::create((role.id, permission.path.clone(), resource.id.clone()), &conn)?;
        assert_eq!(accessible.resource_id, resource.id.clone());

        // 名前の違うリソースを登録する
        let resource: Resource = Resource::create("xxx resource".to_string(), &conn)?;
        let accessible = Accessible::create((role.id, permission.path.clone(), resource.id.clone()), &conn)?;
        assert_eq!(accessible.resource_id, resource.id.clone());

        Ok(())
    });
}
