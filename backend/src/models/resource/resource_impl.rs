use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use crate::db::DBConnection;
use crate::models::{traits::*, Resource};
use crate::types::DieselError;

impl Save<Resource> for Resource {
    fn save(&self, conn: &DBConnection) -> bool {
        // created_at 上書きされないようにしたいけど、とりあえず放置
        // upsertにはならないっぽい どうしたらいい
        diesel::insert_into(Self::table()).values(self).execute(conn).is_ok()
    }
}

impl Find<Resource, DieselError, String> for Resource {
    fn find(id: String, conn: &DBConnection) -> Result<Resource, DieselError> {
        Self::table().find(id).first::<Resource>(conn)
    }
}

#[test]
fn test_resource_impls() {
    use crate::db::connect;
    let conn = connect().get().expect("need connection");
    conn.test_transaction::<_, diesel::result::Error, _>(|| {
        let resource = Resource::create("red_drink_main_server".to_string(), &conn).expect("could not create resource");
        println!("{:?}", resource);
        assert_eq!(
            Resource::find(resource.id.clone(), &conn).map(|resource| (resource.id, resource.name, resource.description)),
            Ok((resource.id, resource.name, resource.description))
        );
        Ok(())
    });   
}