use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use chrono::{Utc, NaiveDateTime};
use crate::schema::{permissions};

#[table_name = "permissions"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug)]
#[primary_key(path)]
pub struct Permission {
    path: String,
    name: String,
    description: String,
    created_at: NaiveDateTime,
}