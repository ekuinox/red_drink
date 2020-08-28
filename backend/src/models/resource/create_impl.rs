use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use diesel::result::Error as DieselError;
use uuid::Uuid;
use crate::db::DBConnection;
use crate::models::{traits::*, Resource};
use crate::schema::resources;

#[table_name = "resources"]
#[derive(Serialize, Deserialize, AsChangeset, Insertable, Identifiable, Associations, Queryable, PartialEq, Clone, Debug, Default)]
#[primary_key(id)]
struct ResourceBuilder {
    id: Option<String>,
    name: String,
    description: Option<String>
}

impl ResourceBuilder {
    fn with_name(name: String) -> ResourceBuilder {
        ResourceBuilder { name, ..ResourceBuilder::default() }
    }
    fn id(self, id: String) -> ResourceBuilder {
        ResourceBuilder { id: Some(id), ..self }
    }
    fn description(self, description: String) -> ResourceBuilder {
        ResourceBuilder { description: Some(description), ..self }
    }
    fn build(self) -> ResourceBuilder {
        ResourceBuilder {
            id: Some(self.id.unwrap_or(Uuid::new_v4().to_string())),
            ..self
        }
    }
    fn save(self, conn: &DBConnection) -> Result<Resource, DieselError> {
        let id = diesel::insert_into(Self::table())
            .values(self.build())
            .returning(resources::id)
            .get_result::<String>(conn)?;
        Self::table().find(id).first::<Resource>(conn)
    }
}

impl Create<Resource, DieselError, String> for Resource {
    fn create(name: String, conn: &DBConnection) -> Result<Resource, DieselError> {
        ResourceBuilder::with_name(name).save(conn)
    }
}

impl Create<Resource, DieselError, (String, String)> for Resource {
    fn create((name, description): (String, String), conn:  &DBConnection) -> Result<Resource, DieselError> {
        ResourceBuilder::with_name(name).description(description).save(conn)
    }
}

/// with id
impl Create<Resource, DieselError, (String, String, String)> for Resource {
    fn create((name, description, id): (String, String, String), conn: &DBConnection) -> Result<Resource, DieselError> {
        ResourceBuilder::with_name(name).description(description).id(id).save(conn)
    }
}

#[test]
fn test_create_with_name() {
    use crate::db::connect;
    let conn = connect().get().expect("need connection");
    conn.test_transaction::<_, diesel::result::Error, _>(|| {
        let resource = Resource::create("resource 1".to_string(), &conn)?;
        assert_eq!(resource.name, "resource 1".to_string());
        Ok(())
    });
}