use diesel::{self, RunQueryDsl};
use diesel::result::Error as DieselError;
use crate::db::DBConnection;
use crate::models::{traits::*, Role};
use crate::schema::roles;
use super::{Policy, Policies};

#[table_name = "roles"]
#[derive(Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug, Clone)]
struct RoleBuilder {
    pub name: String,
    pub policies: Policies
}

impl RoleBuilder {
    fn new(name: String) -> RoleBuilder {
        RoleBuilder { name, policies: Default::default() }
    }
    fn policies(self, policies: Policies) -> RoleBuilder {
        RoleBuilder { policies, ..self }
    }
    fn save(self, connection: &DBConnection) -> Result<Role, DieselError> {
        diesel::insert_into(roles::table)
            .values(self)
            .returning((roles::id, roles::name, roles::created_at, roles::policies))
            .get_result(connection)
            .map(|(id, name, created_at, policies)| Role { id, name, created_at, policies })
    }
}

impl Create<Role, DieselError, String> for Role {
    fn create(name: String, conn: &DBConnection) -> Result<Role, DieselError> {
        RoleBuilder::new(name).save(conn)
    }
}

impl Create<Role, DieselError, (String, Policies)> for Role {
    fn create((name, policies): (String, Policies), conn:  &DBConnection) -> Result<Role, DieselError> {
        RoleBuilder::new(name).policies(policies).save(conn)
    }
}

impl Create<Role, DieselError, (String, Policy)> for Role {
    fn create((name, policy): (String, Policy), conn:  &DBConnection) -> Result<Role, DieselError> {
        Self::create((name, Policies::from(policy)), conn)
    }
}
