use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use chrono::NaiveDateTime;
use crate::schema::{roles, assignments};
use crate::models::traits::*;
use crate::models::permission::Permission;
use crate::models::Accessible;

mod create_impl;

#[table_name = "roles"]
#[derive(Identifiable, AsChangeset, Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug)]
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
        let query = assignments::table.inner_join(roles::table).filter(assignments::user_id.eq(user_id));
        let r = query.select((roles::id, roles::name, roles::created_at)).load::<(i32, String, NaiveDateTime)>(connection);
        r.map(|v| {
            v.into_iter().map(&Role::new_from_tuple).collect()
        }).ok()
    }

    /**
     * Roleに紐づくPermissionを取得する
     */
    pub fn get_permissions(&self, resource_id: Option<String>, conn: &DBConnection) -> Option<Vec<Permission>> {
        match resource_id {
            Some(id) => Accessible::get_permissions(self.id, id, conn),
            None => Accessible::get_permissions_for_root(self.id, conn)
        }.ok()
    }

    pub fn find(id: i32, connection: &DBConnection) -> Option<Role> {
        roles::table.find(id).first::<Role>(connection).ok()
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
    pub fn all(connection: &DBConnection) -> Option<Vec<Role>> {
        roles::table.load::<Role>(connection).ok()
    }
}
