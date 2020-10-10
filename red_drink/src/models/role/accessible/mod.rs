use diesel::sql_types::*;
use diesel::serialize;
use diesel::deserialize;
use diesel::backend::Backend;
use std::io::Write;
use diesel::pg::Pg;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Allowed {
    resources: Vec<String>,
    permissions: Vec<String>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Denied {
    resources: Vec<String>,
    permissions: Vec<String>
}

#[derive(FromSqlRow, Serialize, Deserialize, AsExpression, PartialEq, Debug, Default, Clone)]
#[sql_type = "Jsonb"]
pub struct Accessible {
    allowed: Allowed,
    denied: Denied
}

impl serialize::ToSql<Jsonb, Pg> for Accessible
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        out.write_all(&[1])?;
        serde_json::to_writer(out, self)
            .map(|_| serialize::IsNull::No)
            .map_err(Into::into)
    }
}

impl deserialize::FromSql<Jsonb, Pg> for Accessible
{
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
