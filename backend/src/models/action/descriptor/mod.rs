use diesel::sql_types::*;
use diesel::serialize;
use diesel::deserialize;
use diesel::backend::Backend;
use std::io::Write;
use diesel::pg::Pg;

mod eval;

pub use eval::EvalDescriptor;

#[derive(FromSqlRow, Serialize, Deserialize, AsExpression, PartialEq, Debug, Clone)]
#[sql_type = "Json"]
pub enum Descriptor {
    Eval(EvalDescriptor)
}

pub trait AsKind {
    fn kind() -> String;
}

pub trait AsDescriptor {
    fn as_descriptor(self) -> Descriptor;
}

impl serialize::ToSql<Json, Pg> for Descriptor
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        match serde_json::to_string(self) {
            Ok(json) => {
                let _ = out.write_all(json.as_bytes());
                Ok(serialize::IsNull::No)
            },
            Err(err) => Err(Box::new(err))
        }
    }
}

impl deserialize::FromSql<Json, Pg> for Descriptor
{
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        match serde_json::from_slice::<Self>(bytes.unwrap_or(b"")) {
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
                required_permissons: vec!["*".to_string()],
                ..Default::default()
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