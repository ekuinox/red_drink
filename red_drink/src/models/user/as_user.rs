use crate::db::DBConnection;
use crate::types::DieselError;
use crate::models::traits::Find;
use super::User;

/// ユーザに変換するために
pub trait AsUser<Error> {
    fn as_user(self, conn: &DBConnection) -> Result<User, Error>;
}

/// i32をUserのidとみなして変換する
impl AsUser<DieselError> for i32 {
    fn as_user(self, conn: &DBConnection) -> Result<User, DieselError> {
        User::find(self, conn)
    }
}
