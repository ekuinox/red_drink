use diesel;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::db::DBConnection;
use crate::models::{traits::*, Role};
use crate::schema::roles;

#[table_name = "roles"]
#[derive(Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug, Clone)]
struct RoleBuilder {
    pub name: String
}

impl RoleBuilder {
    fn new(name: String) -> RoleBuilder {
        RoleBuilder { name }
    }
    
    fn save(self, connection: &DBConnection) -> Result<Role, DieselError> {
        diesel::insert_into(roles::table)
            .values(self)
            .returning((roles::id, roles::name, roles::created_at))
            .get_result(connection)
            .map(|(id, name, created_at)| Role { id, name, created_at })
    }
}

impl Create<Role, DieselError, String> for Role {
    fn create(name: String, conn: &DBConnection) -> Result<Role, DieselError> {
        RoleBuilder::new(name).save(conn)
    }
}
