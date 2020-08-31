use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use crate::schema::permissions;
use crate::models::{Permission, traits::*};
use crate::types::DieselError;

impl Find<Permission, DieselError, String> for Permission {
    fn find(path: String, connection: &DBConnection) -> Result<Permission, DieselError> {
        permissions::table.find(path).first::<Permission>(connection)
    }
}
