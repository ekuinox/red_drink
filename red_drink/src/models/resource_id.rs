use super::traits::Find;
use crate::db::DBConnection;
use crate::types::DieselError;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ResourceId(pub String, pub String);

pub trait ToResourceId<T> {
    fn to_resource_id(&self) -> ResourceId;
}

pub trait Resource {
    fn get_type() -> String;
    fn get_id(&self) -> String;
}

impl <T: Resource> ToResourceId<T> for T {
    fn to_resource_id(&self) -> ResourceId {
        ResourceId(T::get_type(), self.get_id())
    }
}

pub trait FromResourceId<T> {
    fn from_resource_id(resource_id: ResourceId, conn: &DBConnection) -> Option<T>;
}

impl <T: Find<T, DieselError, String> + Resource> FromResourceId<T> for T {
    fn from_resource_id(resource_id: ResourceId, conn: &DBConnection) -> Option<T> {
        let ResourceId(resource_type, id) = resource_id;
        if resource_type == T::get_type() {
            T::find(id, conn).ok()
        } else {
            None
        }
    }
}
