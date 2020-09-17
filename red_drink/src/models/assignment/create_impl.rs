use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use crate::db::DBConnection;
use chrono::naive::NaiveDateTime;
use crate::models::{traits::*, Assignment, User, Role};
use crate::schema::assignments;
use crate::types::DieselError;

#[table_name = "assignments"]
#[derive(Serialize, Deserialize, Identifiable, Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Role, foreign_key = "role_id")]
#[primary_key(user_id, role_id)]
pub struct AssignmentBuilder {
    pub user_id: i32,
    pub role_id: i32
}

impl AssignmentBuilder {
    fn new(user_id: i32, role_id: i32) -> AssignmentBuilder {
        AssignmentBuilder { user_id, role_id }
    }
    fn save(self, conn: &DBConnection) -> Result<Assignment, DieselError> {
        diesel::insert_into(Self::table())
            .values(self)
            .returning((assignments::user_id, assignments::role_id, assignments::created_at))
            .get_result::<(i32, i32, NaiveDateTime)>(conn)
            .map(|(user_id, role_id, created_at)| Assignment { user_id, role_id, created_at })
    }
}

impl Create<Assignment, DieselError, (i32, i32)> for Assignment {
    fn create((user_id, role_id): (i32, i32), conn: &DBConnection) -> Result<Assignment, DieselError> {
        AssignmentBuilder::new(user_id, role_id).save(conn)
    }
}
