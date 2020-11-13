use diesel::sql_types::Jsonb;
use crate::models::resource_id::{ResourceId};
mod policy_impl;

/// foo.bar.bazのようなパスから[*, foo.*, foo.bar.*, foo.bar.baz]なパスの配列を求める
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
        let mut last_splited = last_splited.iter().map(|str| str.to_string()).collect::<Vec<String>>();
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

/// required を permissions が持つかチェックする
fn has_permission(required: &String, permissions: &Vec<String>) -> bool {
    let paths = get_parent_paths(&required);
    permissions.into_iter().any(|permission| paths.contains(&permission))
}

/// 許可されているリソース
#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Allowed {
    resources: Vec<ResourceId>,
    permissions: Vec<String>
}

impl Allowed {
    /// 対象のリソースは許可されているか取得する
    pub fn is_allowed(&self, resource: &ResourceId, permission: &String) -> bool {
        has_permission(permission, &self.permissions)
            && self.resources.iter().any(|resource_id| resource_id == resource)
    }
}

/// 拒否されているリソース
#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Denied {
    resources: Vec<ResourceId>,
    permissions: Vec<String>
}

impl Denied {
    /// 対象のリソースは拒否されているか取得する
    pub fn is_denied(&self, resource: &ResourceId, permission: &String) -> bool {
        has_permission(permission, &self.permissions)
            || self.resources.iter().any(|resource_id| resource_id == resource)
    }
}

/// 許可されているものと拒否されているもの
#[derive(FromSqlRow, Serialize, Deserialize, AsExpression, PartialEq, Debug, Default, Clone)]
#[sql_type = "Jsonb"]
pub struct Policy {
    allowed: Allowed,
    denied: Denied
}

impl Policy {
    /// 対象のリソースに対するアクセス権があるか
    pub fn is_allowed(&self, resource: ResourceId, permission: String) -> bool {
        !self.denied.is_denied(&resource, &permission)
            && self.allowed.is_allowed(&resource, &permission)
    }
}

#[test]
fn test_has_permission() {
    let resource = ResourceId("xxx".to_string(), "1234".to_string());
    let policy = Policy::with_allowed(Allowed {
        permissions: vec!["foo.*"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>(),
        resources: vec![resource.clone()]
    }).denied(Denied {
        permissions: vec!["foo.xxx.*"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>(),
        resources: vec![]
    });
    assert!(policy.is_allowed(resource.clone(), "foo.bar".to_string()));
    assert!(policy.is_allowed(resource.clone(), "foo.*".to_string()));
    assert!(!policy.is_allowed(resource.clone(), "foo.xxx.zzz".to_string()));
    assert!(!policy.is_allowed(resource.clone(), "bar.*".to_string()));
}
