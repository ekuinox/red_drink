use diesel::sql_types::*;
use diesel::serialize;
use diesel::deserialize;
use diesel::backend::Backend;
use std::io::Write;

#[derive(FromSqlRow, AsExpression, Serialize, Deserialize, AsExpression, PartialEq, Debug, Clone)]
pub struct Descriptor {
    owner: i32 // User#id
}

impl <DB> serialize::ToSql<Json, DB> for Descriptor
where
    DB: Backend,
    String: serialize::ToSql<Json, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        match serde_json::to_string(self).map(|s| s.to_sql(out)) {
            Ok(a) => a,
            Err(err) => Err(Box::new(err))
        }
    }
}

impl <DB> deserialize::FromSql<Json, DB> for Descriptor
where
    DB: Backend,
    String: deserialize::FromSql<Json, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match serde_json::from_str::<Self>(String::from_sql(bytes)?.to_string().as_str()) {
            Ok(a) => Ok(a),
            Err(err) => Err(Box::new(err))
        }
    }
}
