use chrono::NaiveDateTime;
use crate::schema::actions;

mod descriptor;

#[table_name = "actions"]
#[derive(Serialize, Deserialize, Identifiable, Insertable, Associations, Queryable, PartialEq, Clone, Debug)]
#[primary_key(id)]
pub struct Action {
    id: i32,
    kind: String,
    descriptor: descriptor::Descriptor,
    created_at: NaiveDateTime
}
