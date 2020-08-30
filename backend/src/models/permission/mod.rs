use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use chrono::NaiveDateTime;
use crate::schema::{permissions};

#[table_name = "permissions"]
#[derive(AsChangeset, Serialize, Deserialize, Identifiable, Insertable, Queryable, PartialEq, Eq, Hash, Clone, Debug)]
#[primary_key(path)]
#[primary_key(path)]
pub struct Permission {
    pub path: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

/**
 * foo.bar.bazのようなパスから[*, foo.*, foo.bar.*, foo.bar.baz]なパスの配列を求める
 */
fn get_parent_paths(path: &String) -> Vec<String> {
    let splited = {
        let mut splited = path.split(".").collect::<Vec<&str>>();
        splited.pop();
        splited
    };
    let splited = splited;
    let mut result = splited.into_iter().fold(vec![String::from("*")], |accumrator, current| {
        let last = accumrator.last().unwrap().clone().to_string();
        let last_splited: Vec<&str> = last.split(".").collect();
        // [*] || ["foo.*"]
        let mut last_splited = last_splited.iter().map
        (|str| str.to_string()).collect::<Vec<String>>();
        // [*, "foo.*"] || [*, "foo.*", "foo.bar.*"]
        last_splited.insert(last_splited.len() - 1, String::from(current));
        [&accumrator[..], &vec![last_splited.join(".")][..]].concat()
    });
    result.push(path.clone());
    result.into_iter().fold(Vec::<String>::new(), |mut accumurator, current| {
        if !accumurator.contains(&current) {
            accumurator.push(current);
        }
        accumurator
    })
}

#[test]
fn test_get_parent_paths() {
    let to_string_vec = |v: Vec<&str>| v.into_iter().map(|str| str.to_string()).collect::<Vec<String>>();
    assert_eq!(get_parent_paths(&"foo.bar.baz".to_string()), to_string_vec(vec!["*", "foo.*", "foo.bar.*", "foo.bar.baz"]));
    assert_eq!(get_parent_paths(&"foo.*".to_string()), to_string_vec(vec!["*", "foo.*"]));
    assert_eq!(get_parent_paths(&"*".to_string()), to_string_vec(vec!["*"]));
}

impl Permission {
    pub fn find(path: String, connection: &DBConnection) -> Option<Permission> {
        permissions::table.find(path).get_result::<Permission>(connection).ok()
    }
}

pub(crate) trait HasPermission<T, R> {
    /**
     * Permission配列に欲しいPermissionが含まれているか
     */
    fn has_permission(permissions: T, required: R) -> bool;
}

impl HasPermission<&Vec<String>, &String> for Permission {
    fn has_permission(permissions: &Vec<String>, required: &String) -> bool {
        let paths = get_parent_paths(&required);
        permissions.into_iter().any(|permission| paths.contains(&permission))
    }
}

impl HasPermission<&Vec<Permission>, String> for Permission {
    fn has_permission(permissions: &Vec<Permission>, required: String) -> bool {
        Permission::has_permission(&permissions.iter().map(|permissions| permissions.path.clone()).collect::<Vec<String>>(), &required)
    }
}

#[test]
fn test_has_permission() {
    assert!(Permission::has_permission(&vec!["foo.*".to_string()], &"foo.bar".to_string()));
    assert!(!Permission::has_permission(&vec!["foo.*".to_string()], &"xxx.yyy".to_string()));
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