use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use crate::db::DBConnection;
use crate::models::{traits::*, Action, descriptor::*, Executable, ExecutableContext, ExecutableError};
use crate::types::DieselError;
use crate::schema::actions;

impl Find<Action, DieselError, i32> for Action {
    fn find(id: i32, conn: &DBConnection) -> Result<Action, DieselError> {
        Action::table().find(id).first::<Action>(conn)
    }
}

#[derive(Serialize, Deserialize, Insertable, PartialEq, Clone, Debug)]
#[table_name = "actions"]
struct NewAction {
    kind: String,
    descriptor: Descriptor
}

/// create from descriptor
impl <D: AsDescriptor + AsKind> Create<Action, DieselError, D> for Action {
    fn create(descriptor: D, conn:  &DBConnection) -> Result<Action, DieselError> {
        diesel::insert_into(actions::table)
            .values(NewAction { kind: D::kind(), descriptor: descriptor.as_descriptor() })
            .returning((actions::id, actions::kind, actions::descriptor, actions::created_at))
            .get_result::<Action>(conn)
    }
}

impl Executable<()> for Action {
    fn execute(&self, ctx: ExecutableContext) -> Result<(), ExecutableError> {
        match &self.descriptor {
            Descriptor::Eval(eval) => eval.execute(ctx)
        }
    }
}

#[test]
fn test_create_action() {
    use crate::db::connect;
    
    let conn = connect().get().expect("could not establish connection");
    conn.test_transaction::<_, diesel::result::Error, _>(|| {
        let descriptor = EvalDescriptor {
            command: "./execute.sh".to_string(),
            required_permissons: vec!["*".to_string()]
        };
        let action = Action::create(descriptor.clone(), &conn)?;
        assert_eq!(descriptor.as_descriptor(), action.descriptor);
        Ok(())
    });
}
