use chrono::NaiveDateTime;
use crate::schema::actions;

mod kind;

#[table_name = "actions"]
#[derive(Serialize, Deserialize, Identifiable, Associations, Queryable, PartialEq, Clone, Debug)]
#[primary_key(id)]
pub struct Action {
    id: i32,
    kind: String,
    descriptor: kind::RunCommand,
    created_at: NaiveDateTime
}
