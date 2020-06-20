use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use chrono::NaiveDateTime;
use crate::schema::{roles, roles_permissions, users_roles};
use crate::models::permission::Permission;

#[table_name = "roles"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug)]
#[primary_key(id)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime
}

impl Role {
    /**
     * Roleを作成する
     */
    pub fn new(id: i32, name: String, created_at: NaiveDateTime) -> Role {
        Role {
            id: id,
            name: name,
            created_at: created_at
        }
    }

    /**
     * tupleからRoleを作成する
     */
    pub fn new_from_tuple(t: (i32, String, NaiveDateTime)) -> Role {
        Self::new(t.0, t.1, t.2)
    }

    /**
     * Userに紐づくRoleを取得する
     */
    pub fn get_roles(user_id: i32, connection: &DBConnection) -> Option<Vec<Role>> {
        let query = users_roles::table.inner_join(roles::table).filter(users_roles::user_id.eq(user_id));
        let r = query.select((roles::id, roles::name, roles::created_at)).load::<(i32, String, NaiveDateTime)>(connection);
        r.map(|v| {
            v.into_iter().map(&Role::new_from_tuple).collect()
        }).ok()
    }

    pub fn find(id: i32, connection: &DBConnection) -> Option<Role> {
        roles::table.find(id).get_result::<Role>(connection).ok()
    }

    /**
     * RoleにPermissionを紐付ける
     */
    pub fn attach_permission(&self, permission_path: String, connection: &DBConnection) -> QueryResult<usize> {
        diesel::insert_into(roles_permissions::table).values((
            roles_permissions::role_id.eq(self.id), roles_permissions::permission_path.eq(permission_path))
        ).execute(connection)
    }
}

#[table_name = "roles"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug, Clone)]
#[primary_key(id)]
pub struct RoleInsertable {
    pub name: String
}

impl RoleInsertable {
    pub fn new(name: String) -> RoleInsertable {
        RoleInsertable { name }
    }
    
    /**
     * Roleを新規作成する
     */
    pub fn create(&self, connection: &DBConnection) -> Option<Role> {
        diesel::insert_into(roles::table)
            .values((*self).clone())
            .returning(roles::id)
            .get_result(connection)
            .ok()
            .and_then(|id| {
                Role::find(id, connection)
            })
    }
}

#[table_name = "roles_permissions"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Role, foreign_key = "role_id")]
#[belongs_to(Permission, foreign_key = "permission_path")]
#[primary_key(role_id, permission_path)]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_path: String,
    pub created_at: NaiveDateTime
}
