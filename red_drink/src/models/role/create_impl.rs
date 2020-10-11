use diesel;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::db::DBConnection;
use crate::models::{traits::*, Role};
use crate::schema::roles;
use super::accessible::Accessible;

#[table_name = "roles"]
#[derive(Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug, Clone)]
struct RoleBuilder {
    pub name: String,
    pub accessible: Accessible
}

impl RoleBuilder {
    fn new(name: String) -> RoleBuilder {
        RoleBuilder { name, accessible: Default::default() }
    }
    fn accessible(self, accessible: Accessible) -> RoleBuilder {
        RoleBuilder { accessible, ..self }
    }
    fn save(self, connection: &DBConnection) -> Result<Role, DieselError> {
        diesel::insert_into(roles::table)
            .values(self)
            .returning((roles::id, roles::name, roles::created_at, roles::accessible))
            .get_result(connection)
            .map(|(id, name, created_at, accessible)| Role { id, name, created_at, accessible })
    }
}

impl Create<Role, DieselError, String> for Role {
    fn create(name: String, conn: &DBConnection) -> Result<Role, DieselError> {
        RoleBuilder::new(name).save(conn)
    }
}

impl Create<Role, DieselError, (String, Accessible)> for Role {
    fn create((name, accessible): (String, Accessible), conn:  &DBConnection) -> Result<Role, DieselError> {
        RoleBuilder::new(name).accessible(accessible).save(conn)
    }
}
