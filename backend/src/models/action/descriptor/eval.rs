use std::process::Command;
use super::super::*;
use descriptor::*;

/// Run command red_drink server local
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EvalDescriptor {
    pub shell: String,
    pub command: String,
    pub required_permissons: Vec<String>,
}

impl Default for EvalDescriptor {
    fn default() -> Self {
        Self {
            shell: "bash".to_string(),
            command: "echo hello".to_string(),
            required_permissons: vec![]
        }
    }
}

impl AsDescriptor for EvalDescriptor {
    fn as_descriptor(self) -> Descriptor {
        Descriptor::Eval(self)
    }
}

impl AsKind for EvalDescriptor {
    fn kind() -> String {
        "eval_descriptor".to_string()
    }
}

impl Executable<std::process::Output> for EvalDescriptor {
    fn execute(&self, ctx: &ExecutableContext) -> Result<std::process::Output, ExecutableError> {
        if self.is_allowed(ctx) {
            Command::new(self.shell.clone())
            .arg("-c")
            .arg(self.command.clone())
            .output()
            .map_err(|err| ExecutableError::IOError(err))
        } else {
            Err(ExecutableError::AccessDenied)
        }
    }
    fn is_allowed(&self, ctx: &ExecutableContext) -> bool {
        self.required_permissons.is_empty() || !self.required_permissons.iter()
            .any(|required| !ctx.executor.has_permission(required.to_owned().to_owned(), None, ctx.conn))
    }
}

#[test]
fn test_eval_command() {
    use diesel::prelude::*;
    use crate::models::{User, user::GitHubAccountDetail};
    use crate::db::connect;

    let detail = GitHubAccountDetail {
        id: 1,
        name: "Foo Taro".to_string(),
        avatar_url: "avatar_url".to_string(),
        email: "foo@example.com".to_string(),
        login: "foo".to_string()
    };

    let conn = connect().get().expect("could not established connection");
    conn.test_transaction::<_, (), _>(|| {
        let user = User::create_with_github_detail(detail.clone(), &conn)?;
        let r = user.add_role(0, &conn);
        assert!(r);

        let eval = EvalDescriptor {
            command: "echo hello".to_string(),
            required_permissons: vec!["*".to_owned()],
            ..Default::default()
        };
        let ctx = ExecutableContext { executor: &user, conn: &conn };
        assert!(eval.is_allowed(&ctx));
        assert!(eval.execute(&ctx).is_ok());

        Ok(())
    });
}
