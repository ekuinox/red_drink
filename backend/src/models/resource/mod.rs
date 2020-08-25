use chrono::NaiveDateTime;
use crate::schema::resources;

mod resource_impl;

#[table_name = "resources"]
#[derive(Serialize, Deserialize, AsChangeset, Insertable, Identifiable, Associations, Queryable, PartialEq, Clone, Debug)]
#[primary_key(id)]
pub struct Resource {
    id: String,
    name: String,
    description: String,
    created_at: NaiveDateTime
}
