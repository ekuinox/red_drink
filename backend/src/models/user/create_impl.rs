use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use chrono::NaiveDateTime;
use crate::models::{User, traits::*};
use crate::db::DBConnection;
use crate::types::DieselError;
use crate::schema::users;


#[table_name = "users"]
#[derive(Serialize, Deserialize, Insertable, Identifiable, Associations, Queryable, PartialEq, Clone, Copy, Debug)]
#[primary_key(id)]
struct UserBuilder {
    id: Option<i32>
}

impl UserBuilder {
    fn new(id: Option<i32>) -> UserBuilder {
        UserBuilder { id }
    }
    fn save(self, conn: &DBConnection) -> Result<User, DieselError> {
        diesel::insert_into(Self::table())
            .values(self)
            .returning((users::id, users::created_at))
            .get_result::<(i32, NaiveDateTime)>(conn)
            .map(|(id, created_at)| User { id, created_at })
    }
}

impl Create<User, DieselError, ()> for User {
    fn create(_: (), conn: &DBConnection) -> Result<User, DieselError> {
        UserBuilder::new(None).save(conn)
    }
}

impl Create<User, DieselError, i32> for User {
    fn create(id: i32, conn: &DBConnection) -> Result<User, DieselError> {
        UserBuilder::new(Some(id)).save(conn)
    }
}