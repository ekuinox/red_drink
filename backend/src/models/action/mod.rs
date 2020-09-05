use chrono::NaiveDateTime;
use crate::schema::actions;
use crate::models::{User};
use crate::db::DBConnection;

mod action_impl;
pub mod descriptor;

#[table_name = "actions"]
#[derive(Serialize, Deserialize, Identifiable, Insertable, Associations, Queryable, PartialEq, Clone, Debug)]
#[primary_key(id)]
pub struct Action {
    id: i32,
    kind: String,
    descriptor: descriptor::Descriptor,
    created_at: NaiveDateTime
}

pub struct ExecutableContext<'a> {
    pub executor: &'a User,
    pub conn: &'a DBConnection
}

#[derive(Debug)]
pub enum ExecutableError {
    AccessDenied,
    IOError(std::io::Error),
    DieselError(crate::types::DieselError)
}

pub trait Executable<T> {
    fn execute(&self, ctx: ExecutableContext) -> Result<T, ExecutableError>;
}