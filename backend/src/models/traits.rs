use crate::db::DBConnection;

/// create model without save to db
pub trait New<T, Args> {
    fn new(args: Args) -> T;
}

/// create model with save to db
pub trait Create<T, E, Args> {
    fn create(args: Args, conn:  &DBConnection) -> Result<T, E>;
}

/// save model
pub trait Save<T> {
    fn save(&self, conn: &DBConnection) -> bool;
}

/// find model
pub trait Find<T, Id> {
    fn find(id: Id, conn: &DBConnection) -> Option<T>;
}
