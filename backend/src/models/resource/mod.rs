use chrono::NaiveDateTime;
use crate::schema::resources;

mod resource_impl;
mod create_impl;

#[table_name = "resources"]
#[derive(Serialize, Deserialize, AsChangeset, Insertable, Identifiable, Associations, Queryable, PartialEq, Clone, Debug)]
#[primary_key(id)]
pub struct Resource {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime
}
