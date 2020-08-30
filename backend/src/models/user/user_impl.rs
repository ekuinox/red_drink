use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use crate::schema::users;
use crate::types::DieselError;
use crate::models::{User, traits::*};

impl Find<User, DieselError, i32> for User {
    fn find(id: i32, connection: &DBConnection) -> Result<User, DieselError> {
        users::table.find(id).first::<User>(connection)
    }
}

/// ユーザの操作
impl User {
    /// get all users
    pub fn all(connection: &DBConnection) -> Option<Vec<User>> {
        users::table.load::<User>(connection).ok()
    }
}
