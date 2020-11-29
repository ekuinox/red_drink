use diesel::sql_types::*;
use diesel::serialize;
use diesel::deserialize;
use diesel::backend::Backend;
use std::io::Write;
use diesel::pg::Pg;

mod eval;

pub use eval::EvalDescriptor;

#[derive(FromSqlRow, Serialize, Deserialize, AsExpression, PartialEq, Debug, Clone)]
#[sql_type = "Jsonb"]
pub enum Descriptor {
    Eval(EvalDescriptor)
}

pub trait AsKind {
    fn kind() -> String;
}

pub trait AsDescriptor {
    fn as_descriptor(self) -> Descriptor;
}

impl serialize::ToSql<Jsonb, Pg> for Descriptor
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        out.write_all(&[1])?;
        serde_json::to_writer(out, self)
            .map(|_| serialize::IsNull::No)
            .map_err(Into::into)
    }
}

impl deserialize::FromSql<Jsonb, Pg> for Descriptor
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

#[test]
fn test_descriptor() {
    use diesel::prelude::*;
    use chrono::prelude::*;
    use crate::db::connect;
    use crate::schema::actions;
    use crate::models::Action;
    
    let conn = connect().get().expect("could not establish conneting");
    conn.test_transaction::<_, diesel::result::Error, _>(|| {
        use crate::models::resource_id::ROOT_RESOURCE;
        let action1 = Action {
            id: 0,
            kind: "kind".to_string(),
            descriptor: EvalDescriptor {
                command: "echo hello".to_string(),
                requires: vec![(
                    vec![ROOT_RESOURCE.clone()],
                    vec!["*".to_owned()]
                )],
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