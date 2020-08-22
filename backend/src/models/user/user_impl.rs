use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use crate::schema::users;
use crate::models::user::User;
use crate::models::user::UserInsertable;

/// ユーザの操作
impl User {

    /// 新規登録
    pub fn create(user: UserInsertable, connection: &DBConnection) -> Option<User> {
        diesel::insert_into(users::table).values(&user).execute(connection)
            .map(|_| {
                users::table.order(users::id.desc()).first(connection).ok()
            })
            .ok()
            .flatten()
    }

    /// ユーザIDから取得する
    pub fn find(id: i32, connection: &DBConnection) -> Option<User> {
        users::table.find(id).get_result(connection).ok()
    }


    /// get all users
    pub fn all(connection: &DBConnection) -> Option<Vec<User>> {
        users::table.load::<User>(connection).ok()
    }
}
