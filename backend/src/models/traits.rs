use crate::db::DBConnection;

/// new model
pub trait New<T, Args> {
    fn new(args: Args) -> T;
}

/// save model
pub trait Save<T> {
    fn save(&self, conn: &DBConnection) -> bool;
}

/// find model
pub trait Find<T, Id> {
    fn find(id: Id, conn: &DBConnection) -> Option<T>;
}
