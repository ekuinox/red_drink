use super::super::*;
use descriptor::*;

/// Run command red_drink server local
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EvalDescriptor {
    pub command: String,
    pub required_permissons: Vec<String>,
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

impl Executable<()> for EvalDescriptor {
    fn execute(&self, ctx: ExecutableContext) -> Result<(), ExecutableError> {
        let is_denined = self.required_permissons.iter()
            .find(|required| !ctx.executor.has_permission(required.to_owned().to_owned(), None, ctx.conn))
            .is_some();
        if is_denined {
            Err(ExecutableError::AccessDenied)
        } else {
            // todo
            Ok(())
        }
    }
}
