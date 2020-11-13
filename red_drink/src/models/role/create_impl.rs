use diesel::{self, RunQueryDsl};
use diesel::result::Error as DieselError;
use crate::db::DBConnection;
use crate::models::{traits::*, Role};
use crate::schema::roles;
use super::policy::Policy;

#[table_name = "roles"]
#[derive(Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug, Clone)]
struct RoleBuilder {
    pub name: String,
    pub policy: Policy
}

impl RoleBuilder {
    fn new(name: String) -> RoleBuilder {
        RoleBuilder { name, policy: Default::default() }
    }
    fn policy(self, policy: Policy) -> RoleBuilder {
        RoleBuilder { policy, ..self }
    }
    fn save(self, connection: &DBConnection) -> Result<Role, DieselError> {
        diesel::insert_into(roles::table)
            .values(self)
            .returning((roles::id, roles::name, roles::created_at, roles::policy))
            .get_result(connection)
            .map(|(id, name, created_at, policy)| Role { id, name, created_at, policy })
    }
}

impl Create<Role, DieselError, String> for Role {
    fn create(name: String, conn: &DBConnection) -> Result<Role, DieselError> {
        RoleBuilder::new(name).save(conn)
    }
}

impl Create<Role, DieselError, (String, Policy)> for Role {
    fn create((name, policy): (String, Policy), conn:  &DBConnection) -> Result<Role, DieselError> {
        RoleBuilder::new(name).policy(policy).save(conn)
    }
}
