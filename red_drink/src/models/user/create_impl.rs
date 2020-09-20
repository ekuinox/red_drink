use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use chrono::NaiveDateTime;
use crate::models::{User, traits::*};
use crate::db::DBConnection;
use crate::types::DieselError;
use crate::schema::users;


#[table_name = "users"]
#[derive(Serialize, Deserialize, Insertable, Identifiable, Associations, Queryable, PartialEq, Clone, Debug)]
#[primary_key(id)]
struct UserBuilder {
    id: Option<i32>,
    name: String,
    avatar_url: Option<String>,
    email: Option<String>
}

impl UserBuilder {
    fn new(name: String) -> UserBuilder {
        UserBuilder {
            id: None,
            name,
            avatar_url: None,
            email: None
        }
    }
    fn id(self, id: i32) -> UserBuilder {
        UserBuilder { id: Some(id), ..self }
    }
    fn avatar_url(self, avatar_url: String) -> UserBuilder {
        UserBuilder { avatar_url: Some(avatar_url), ..self }
    }
    fn email(self, email: String) -> UserBuilder {
        UserBuilder { email: Some(email), ..self }
    }
    fn save(self, conn: &DBConnection) -> Result<User, DieselError> {
        diesel::insert_into(Self::table())
            .values(self)
            .returning((users::id, users::name, users::avatar_url, users::email, users::created_at))
            .get_result::<(i32, String, Option<String>, Option<String>, NaiveDateTime)>(conn)
            .map(|(id, name, avatar_url, email, created_at)| User { id, name, avatar_url, email, created_at })
    }
}

impl Create<User, DieselError, String> for User {
    fn create(name: String, conn: &DBConnection) -> Result<User, DieselError> {
        UserBuilder::new(name).save(conn)
    }
}

impl Create<User, DieselError, (String, i32)> for User {
    fn create((name, id): (String, i32), conn: &DBConnection) -> Result<User, DieselError> {
        UserBuilder::new(name).id(id).save(conn)
    }
}

impl Create<User, DieselError, (String, String, String)> for User {
    fn create((name, avatar_url, email): (String, String, String), conn: &DBConnection) -> Result<User, DieselError> {
        UserBuilder::new(name).avatar_url(avatar_url).email(email).save(conn)
    }
}
