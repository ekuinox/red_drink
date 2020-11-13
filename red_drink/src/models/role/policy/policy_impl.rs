use diesel::sql_types::*;
use diesel::serialize;
use diesel::deserialize;
use diesel::backend::Backend;
use std::io::Write;
use diesel::pg::Pg;
use super::*;

impl Policy {
    pub fn with_allowed(allowed: Allowed) -> Policy {
        Policy { allowed, denied: Default::default() }
    }
    pub fn with_denies(denied: Denied) -> Policy {
        Policy { denied, allowed: Default::default() }
    }
    pub fn allowed(self, allowed: Allowed) -> Policy {
        Policy { allowed, ..self }
    }
    pub fn denied(self, denied: Denied) -> Policy {
        Policy { denied, ..self }
    }
}

impl serialize::ToSql<Jsonb, Pg> for Policy
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        out.write_all(&[1])?;
        serde_json::to_writer(out, self)
            .map(|_| serialize::IsNull::No)
            .map_err(Into::into)
    }
}

impl deserialize::FromSql<Jsonb, Pg> for Policy
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
