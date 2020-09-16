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

impl Find<User, DieselError, String> for User {
    fn find(username: String, conn: &DBConnection) -> Result<User, DieselError> {
        users::table.filter(users::name.eq(username)).first::<User>(conn)
    }
}

/// ユーザの操作
impl User {
    /// get all users
    pub fn all(connection: &DBConnection) -> Result<Vec<User>, DieselError> {
        users::table.load::<User>(connection)
    }
}
