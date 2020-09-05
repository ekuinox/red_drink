use crate::models::action::descriptor::*;

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
