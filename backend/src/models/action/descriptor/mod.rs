use diesel::sql_types::*;
use diesel::serialize;
use diesel::deserialize;
use diesel::backend::Backend;
use std::io::Write;

mod eval;

pub use eval::EvalDescriptor;

#[derive(FromSqlRow, Serialize, Deserialize, AsExpression, PartialEq, Debug, Clone)]
#[sql_type = "Varchar"]
pub enum Descriptor {
    Eval(eval::EvalDescriptor)
}

pub trait AsKind {
    fn kind() -> String;
}

pub trait AsDescriptor {
    fn as_descriptor(self) -> Descriptor;
}

impl <DB> serialize::ToSql<Varchar, DB> for Descriptor
where
    DB: Backend,
    String: serialize::ToSql<Varchar, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        match serde_json::to_string(self).map(|s| s.to_sql(out)) {
            Ok(a) => a,
            Err(err) => Err(Box::new(err))
        }
    }
}

impl <DB> deserialize::FromSql<Varchar, DB> for Descriptor
where
    DB: Backend,
    String: deserialize::FromSql<Varchar, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match serde_json::from_str::<Self>(String::from_sql(bytes)?.to_string().as_str()) {
            Ok(a) => Ok(a),
            Err(err) => Err(Box::new(err))
        }
    }
}

#[test]
fn test_descriptor() {
    use diesel::prelude::*;
    use chrono::prelude::*;
    use crate::db::connect;
    use crate::schema::actions;
    use crate::models::Action;
    
    let conn = connect().get().expect("could not establish conneting");
    conn.test_transaction::<_, diesel::result::Error, _>(|| {
        let action1 = Action {
            id: 0,
            kind: "kind".to_string(),
            descriptor: EvalDescriptor {
                command: "./execute.sh".to_string(),
                required_permissons: vec!["*".to_string()]
            }.as_descriptor(),
            created_at: Utc::now().naive_utc()
        };
        let action2 = diesel::insert_into(actions::table)
            .values(&action1)
            .returning((actions::id, actions::kind, actions::descriptor, actions::created_at))
            .get_result::<Action>(&conn)?;
        assert_eq!(action1.descriptor, action2.descriptor);

        let action3 = actions::table.find(action1.id).first::<Action>(&conn)?;
        assert_eq!(action1.descriptor, action3.descriptor);

        Ok(())
    });
}