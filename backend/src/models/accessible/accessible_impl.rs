use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use crate::models::{traits::*, Accessible};

impl Find<Accessible, (i32, String, String)> for Accessible {
    fn find(id: (i32, String, String), conn: &DBConnection) -> Option<Accessible> {
        use diesel::associations::HasTable;
        Self::table().find(id).first::<Accessible>(conn).ok()
    }
}
