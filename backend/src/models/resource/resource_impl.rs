use chrono::Utc;
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use crate::db::DBConnection;
use crate::models::{traits::*, Resource};

impl New<Resource, (String, String, String, NaiveDateTime)> for Resource {
    fn new((id, name, description, created_at): (String, String, String, NaiveDateTime)) -> Resource {
        Resource { id, name, description, created_at }
    }
}

impl New<Resource, (String, String, String)> for Resource {
    fn new((id, name, description): (String, String, String)) -> Resource {
        Resource::new((id, name, description, Utc::now().naive_utc()))
    }
}

impl New<Resource, (String, String)> for Resource {
    fn new((id, name): (String, String)) -> Resource {
        Resource::new((id, name, String::default()))
    }
}

impl New<Resource, String> for Resource {
    fn new(name: String) -> Resource {
        Resource::new((name.clone(), name))
    }
}

impl Save<Resource> for Resource {
    fn save(&self, conn: &DBConnection) -> bool {
        // created_at 上書きされないようにしたいけど、とりあえず放置
        // upsertにはならないっぽい どうしたらいい
        diesel::insert_into(Self::table()).values(self).execute(conn).is_ok()
    }
}

impl Find<Resource, String> for Resource {
    fn find(id: String, conn: &DBConnection) -> Option<Resource> {
        Self::table().find(id).first::<Resource>(conn).ok()
    }
}

#[test]
fn test_resource_impls() {
    use crate::db::connect;

    // nameだけでResourceを作る
    let resource = Resource::new("red_drink_main_server".to_string());
        
    assert_eq!(resource.id, "red_drink_main_server");
    assert_eq!(resource.name, "red_drink_main_server");
    assert_eq!(resource.description, "");

    let conn = connect().get().expect("need connection");
    conn.test_transaction::<_, diesel::result::Error, _>(|| {
        assert!(resource.save(&conn));
        // created_atの比較は行わない
        assert_eq!(
            Resource::find("red_drink_main_server".to_string(), &conn).map(|resource| (resource.id, resource.name, resource.description)),
            Some((resource.id, resource.name, resource.description))
        );
        Ok(())
    });   
}