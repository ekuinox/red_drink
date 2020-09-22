use crate::db::DBConnection;

/// create model with save to db
pub trait Create<T, E, Args> {
    fn create(args: Args, conn:  &DBConnection) -> Result<T, E>;
}

/// save model
pub trait Save<T> {
    fn save(&self, conn: &DBConnection) -> bool;
}

/// find model
pub trait Find<T, E, Id> {
    fn find(id: Id, conn: &DBConnection) -> Result<T, E>;
}

pub trait All<T, E> {
    fn all(conn: &DBConnection) -> Result<Vec<T>, E>;
}
