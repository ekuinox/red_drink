use chrono::NaiveDateTime;
use crate::schema::resources;

#[table_name = "resources"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Identifiable, Associations, Queryable, PartialEq, Clone, Debug)]
#[primary_key(id)]
pub struct Resource {
    id: String,
    name: String,
    description: String,
    created_at: NaiveDateTime
}
