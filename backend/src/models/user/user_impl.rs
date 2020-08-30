use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use crate::schema::users;
use crate::models::user::User;
use crate::types::DieselError;

/// ユーザの操作
impl User {

    /// ユーザIDから取得する
    pub fn find(id: i32, connection: &DBConnection) -> Result<User, DieselError> {
        users::table.find(id).first::<User>(connection)
    }


    /// get all users
    pub fn all(connection: &DBConnection) -> Option<Vec<User>> {
        users::table.load::<User>(connection).ok()
    }
}
