use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use crate::models::{traits::*, GitHubAccount, User};
use crate::db::DBConnection;
use crate::types::DieselError;

impl Find<GitHubAccount, DieselError, i32> for GitHubAccount {
    fn find(github_id: i32, conn: &DBConnection) -> Result<GitHubAccount, DieselError> {
        Self::table().find(github_id).first::<GitHubAccount>(conn)
    }
}

impl GitHubAccount {
    /**
     * Userに変換する
     */
    pub fn to_user(self, connection: &DBConnection) -> Result<User, DieselError> {
        User::find(self.user_id, connection)
    }
}
