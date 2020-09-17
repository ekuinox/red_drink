use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use crate::db::DBConnection;
use chrono::naive::NaiveDateTime;
use crate::models::{traits::*, GitHubAccount, User};
use crate::schema::github_accounts;
use crate::types::DieselError;

#[table_name = "github_accounts"]
#[derive(AsChangeset, Serialize, Deserialize, Identifiable, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[primary_key(github_id)]
pub struct GitHubAccountBuilder {
    github_id: i32,
    user_id: i32
}

impl GitHubAccountBuilder {
    fn new(github_id: i32, user_id: i32) -> GitHubAccountBuilder {
        GitHubAccountBuilder { github_id, user_id }
    }
    fn save(self, conn: &DBConnection) -> Result<GitHubAccount, DieselError> {
        diesel::insert_into(Self::table())
            .values(self)
            .returning((github_accounts::github_id, github_accounts::user_id, github_accounts::created_at))
            .get_result::<(i32, i32, NaiveDateTime)>(conn)
            .map(|(github_id, user_id, created_at)| GitHubAccount { github_id, user_id, created_at })
    }
}

impl Create<GitHubAccount, DieselError, (i32, i32)> for GitHubAccount {
    fn create((github_id, user_id): (i32, i32), conn: &DBConnection) -> Result<GitHubAccount, DieselError> {
        GitHubAccountBuilder::new(github_id, user_id).save(conn)
    }
}
