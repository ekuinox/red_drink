use std::process::Command;
use super::super::*;
use descriptor::*;
use crate::models::resource_id::ResourceId;

/// Run command red_drink server local
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EvalDescriptor {
    pub shell: String,
    pub command: String,
    pub resources: Vec<(ResourceId, Vec<String>)>
}

impl Default for EvalDescriptor {
    fn default() -> Self {
        Self {
            shell: "bash".to_string(),
            command: "echo hello".to_string(),
            resources: vec![]
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
        // リソースに対する権限がすべてあればOK
        self.resources.iter().all(|(resource, required_permissions)| {
            required_permissions.iter().all(|required| {
                ctx.executor.has_permission(required.clone(), resource.clone(), ctx.conn)
            })
        })
    }
}

#[test]
fn test_eval_command() {
    use diesel::prelude::*;
    use crate::models::{User, user::GitHubAccountDetail, traits::*, Role, role::policy::{Policy, Allowed}};
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
        use crate::models::resource_id::ROOT_RESOURCE;

        let user = User::create_with_github_detail(detail.clone(), &conn)?;
        let allowed = Allowed {
            resources: vec![ROOT_RESOURCE.clone()],
            permissions: vec!["*".to_string()]
        };
        let role: Role = Role::create(("*".to_string(), Policy::with_allowed(allowed)), &conn).map_err(|_| ())?;
        let r = user.add_role(role.id, &conn);
        assert!(r);

        let eval = EvalDescriptor {
            command: "echo hello".to_string(),
            resources: vec![(ROOT_RESOURCE.clone(), vec!["foo.bar.baz".to_owned()])],
            ..Default::default()
        };
        let ctx = ExecutableContext { executor: &user, conn: &conn };
        assert!(eval.is_allowed(&ctx));
        assert!(eval.execute(&ctx).is_ok());

        Ok(())
    });
}
