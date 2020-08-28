use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use diesel::result::Error as DieselError;
use crate::db::DBConnection;
use crate::models::{traits::*, Accessible, Permission};
use crate::schema::accessibles;

impl Find<Accessible, (i32, String, String)> for Accessible {
    fn find(id: (i32, String, String), conn: &DBConnection) -> Option<Accessible> {
        Self::table().find(id).first::<Accessible>(conn).ok()
    }
}

impl Accessible {
    /// roleが持つ指定したリソースに対しての権限を取得する
    pub fn get_permissions(role_id: i32, resource_id: String, conn: &DBConnection) -> Result<Vec<Permission>, DieselError> {
        Self::table()
            .inner_join(Permission::table())
            .filter(accessibles::role_id.eq(role_id))
            .filter(accessibles::resource_id.eq(resource_id))
            .get_results::<(Self, Permission)>(conn)
            .map(|results| results.into_iter().unzip::<_, _, Vec<_>, Vec<Permission>>().1)
    }
    /// rootリソースに対してget_permissionsを行う
    pub fn get_permissions_for_root(role_id: i32, conn: &DBConnection) -> Result<Vec<Permission>, DieselError> {
        Self::get_permissions(role_id, "*".to_string(), conn)
    }
}
