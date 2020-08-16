use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use chrono::NaiveDateTime;
use crate::schema::{permissions};

#[table_name = "permissions"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, PartialEq, Eq, Hash, Clone, Debug)]
#[primary_key(path)]
pub struct Permission {
    path: String,
    name: String,
    description: Option<String>,
    created_at: NaiveDateTime,
}

fn get_parent_paths(path: &String) -> Vec<String> {
    let splited = path.split(".").filter(|node| *node != "*");
    let mut result = splited.fold(vec![String::from("*")], |accumrator, current| {
        let last = accumrator.last().unwrap().clone().to_string();
        let last_splited: Vec<&str> = last.split(".").collect();
        // [*] || ["foo.*"]
        let mut last_splited = last_splited.iter().map(|str| str.to_string()).collect::<Vec<String>>();
        // [*, "foo.*"] || [*, "foo.*", "foo.bar.*"]
        last_splited.insert(last_splited.len() - 1, String::from(current));
        [&accumrator[..], &vec![last_splited.join(".")][..]].concat()
    });
    result.push(path.clone());
    result
}

#[test]
fn test_get_parent_paths() {
    let path = "foo.bar.baz".to_string();
    let paths = get_parent_paths(&path);
    println!("{:?}", paths);
    println!("{:?}", get_parent_paths(&"foo.xxx.*".to_string()));
}

impl Permission {
    pub fn find(path: String, connection: &DBConnection) -> Option<Permission> {
        permissions::table.find(path).get_result::<Permission>(connection).ok()
    }

    /**
     * Permission配列に欲しいPermissionが含まれているか
     */
    pub fn has_permission(permissions: &Vec<Self>, required: String) -> bool {
        let paths = get_parent_paths(&required);
        true
    }
}

#[table_name = "permissions"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Queryable, PartialEq, Debug, Clone)]
#[primary_key(path)]
pub struct PermissionInsertable {
    path: String,
    name: String,
    description: Option<String>
}

impl PermissionInsertable {
    pub fn new(path: String, name: String, description: Option<String>) -> PermissionInsertable {
        PermissionInsertable {
            path, name, description
        }
    }
    
    pub fn create(&self, connection: &DBConnection) -> Option<Permission> {
        let _ = diesel::insert_into(permissions::table).values((*self).clone()).execute(connection);
        Permission::find(self.path.clone(), connection)
    }
}