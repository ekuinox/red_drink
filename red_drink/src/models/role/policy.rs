use diesel::serialize;
use diesel::deserialize;
use diesel::backend::Backend;
use std::io::Write;
use diesel::pg::Pg;
use super::*;
use diesel::sql_types::Jsonb;
use crate::models::resource_id::ResourceId;

/// アクセスを許可するかどうか
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Accessible {
    Allowed,
    Denied,
}

impl Default for Accessible {
    fn default() -> Self {
        Accessible::Allowed
    }
}

/// リソースに対しての権限
#[derive(FromSqlRow, Serialize, Deserialize, AsExpression, PartialEq, Debug, Default, Clone)]
#[sql_type = "Jsonb"]
pub struct Policy {
    /// 優先度
    pub priority: u32,
    /// 許可するか拒否するか
    pub accessible: Accessible,
    /// 対象のリソースのリスト
    pub resources: Vec<ResourceId>,
    /// 対象の権限
    pub permissions: Vec<Permission>,
}

impl Includes<(String, ResourceId)> for Policy {
    fn includes(&self, (required, resource): (String, ResourceId)) -> bool {
        self.resources.contains(&resource) // 対象のリソースがこのポリシーに含まれているか
        && self.permissions.iter().any(|permission| permission.includes(&required)) // 対象の権限がこのポリシーに含まれているか
    }
}

impl serialize::ToSql<Jsonb, Pg> for Policy {
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        out.write_all(&[1])?;
        serde_json::to_writer(out, self)
            .map(|_| serialize::IsNull::No)
            .map_err(Into::into)
    }
}

impl deserialize::FromSql<Jsonb, Pg> for Policy {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        if bytes[0] != 1 {
            Err("Unsupported JSONB encoding version".into())
        } else {
            match serde_json::from_slice::<Self>(&bytes[1..]) {
                Ok(a) => Ok(a),
                Err(err) => Err(Box::new(err))
            }
        }
    }
}

#[test]
fn test_has_permission() {
    use crate::models::resource_id::ROOT_RESOURCE;
    let p1 = Policy {
        resources: vec![ROOT_RESOURCE.clone()],
        permissions: vec![Permission::from("foo.bar.*"), Permission::from("xxx.*")],
        ..Default::default()
    };
    assert!(p1.includes(("foo.bar.baz".to_string(), ROOT_RESOURCE.clone())));
    assert!(p1.includes(("foo.bar.*".to_string(), ROOT_RESOURCE.clone())));
    assert!(p1.includes(("xxx.yyy.zzz".to_string(), ROOT_RESOURCE.clone())));
    assert!(!p1.includes(("foo.abc.*".to_string(), ROOT_RESOURCE.clone())));
    assert!(!p1.includes(("abc.*".to_string(), ROOT_RESOURCE.clone())));
}
