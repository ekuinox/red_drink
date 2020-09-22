use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use crate::db::DBConnection;
use crate::models::{traits::*, Accessible, Permission};
use crate::schema::accessibles;
use crate::types::DieselError;

impl Find<Accessible, DieselError, (i32, String, String)> for Accessible {
    fn find(id: (i32, String, String), conn: &DBConnection) -> Result<Accessible, DieselError> {
        Self::table().find(id).first::<Accessible>(conn)
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
    /// Roleが持つすべての権限を取得する
    pub fn get_permissions_all_with_resource(role_id: i32, conn: &DBConnection) -> Result<Vec<(String, String)>, DieselError> {
        Self::table()
            .filter(accessibles::role_id.eq(role_id))
            .select((accessibles::permission_path, accessibles::resource_id))
            .get_results::<(String, String)>(conn)
    }
}