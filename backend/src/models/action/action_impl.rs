use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use crate::db::DBConnection;
use crate::models::{traits::*, Action, action::{descriptor::*, *}};
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

impl All<Action, DieselError> for Action {
    fn all(conn: &DBConnection) -> Result<Vec<Action>, DieselError> {
        Action::table().load(conn)
    }
}

impl Action {
    pub fn all_by_kind<D: AsKind>(conn: &DBConnection) -> Result<Vec<Action>, DieselError> {
        Action::table()
            .filter(actions::kind.eq(D::kind()))
            .get_results::<Action>(conn)
    }
    pub fn all_allowed(ctx: &ExecutableContext) -> Result<Vec<Action>, DieselError> {
        Action::all(ctx.conn).map(|actions| 
            actions.into_iter().filter(|action| action.is_allowed(ctx)).collect()
        )
    }
}

impl Executable<()> for Action {
    fn execute(&self, ctx: &ExecutableContext) -> Result<(), ExecutableError> {
        match &self.descriptor {
            Descriptor::Eval(eval) => eval.execute(ctx).map(|_| ())
        }
    }
    fn is_allowed(&self, ctx: &ExecutableContext) -> bool {
        match &self.descriptor {
            Descriptor::Eval(eval) => eval.is_allowed(ctx)
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
            required_permissons: vec!["*".to_string()],
            ..Default::default()
        };
        let action = Action::create(descriptor.clone(), &conn)?;
        assert_eq!(descriptor.as_descriptor(), action.descriptor);
        Ok(())
    });
}

#[test]
fn test_all_action() {
    use crate::db::connect;
    let conn = connect().get().expect("could not establish connection");
    
    conn.test_transaction::<_, diesel::result::Error, _>(|| {
        let descriptor = EvalDescriptor {
            command: "./execute.sh".to_string(),
            required_permissons: vec!["*".to_string()],
            ..Default::default()
        };
        let action = Action::create(descriptor.clone(), &conn)?;

        let actions = Action::all_by_kind::<EvalDescriptor>(&conn)?;

        assert!(actions.into_iter().any(|a| a == action));

        Ok(())
    });
}