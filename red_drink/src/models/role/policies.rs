use std::ops::Deref;
use diesel::serialize;
use diesel::deserialize;
use diesel::backend::Backend;
use std::io::Write;
use diesel::pg::Pg;
use super::*;
use diesel::sql_types::Jsonb;

/// リソースに対しての権限
#[derive(FromSqlRow, Serialize, Deserialize, AsExpression, PartialEq, Debug, Default, Clone)]
#[sql_type = "Jsonb"]
pub struct Policies(pub Vec<Policy>);

impl From<Policy> for Policies {
    fn from(policy: Policy) -> Self {
        Policies(vec![policy])
    }
}

impl Deref for Policies {
    type Target = Vec<Policy>;
    fn deref(&self) -> &Vec<Policy> {
        &self.0
    }
}

impl serialize::ToSql<Jsonb, Pg> for Policies {
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        out.write_all(&[1])?;
        serde_json::to_writer(out, self)
            .map(|_| serialize::IsNull::No)
            .map_err(Into::into)
    }
}

impl deserialize::FromSql<Jsonb, Pg> for Policies {
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
