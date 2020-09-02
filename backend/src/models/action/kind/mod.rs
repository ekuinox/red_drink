use diesel::sql_types::*;
use diesel::serialize::*;
use diesel::deserialize;
use diesel::backend::Backend;
use std::io::Write;

#[derive(Serialize, Deserialize, AsExpression, Debug, Clone)]
pub struct RunCommand {
    cmd: String
}

impl <DB> ToSql<Json, DB> for RunCommand
where
    DB: Backend,
    String: ToSql<Json, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> Result {
        match serde_json::to_string(self).map(|s| s.to_sql(out)) {
            Ok(a) => a,
            Err(err) => Err(Box::new(err))
        }
    }
}
