use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv_codegen::dotenv;

pub type DBConnection = PgConnection;

// An alias to the type for a pool of Diesel Mysql Connection
pub type ConnectionPool = Pool<ConnectionManager<DBConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &str = dotenv!("DATABASE_URL");

/// Initialize the database pool.
pub fn connect() -> ConnectionPool {
    let manager = ConnectionManager::<DBConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("Failed to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub PooledConnection<ConnectionManager<DBConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<ConnectionPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &Connection as an &SqliteConnection.
impl Deref for Connection {
    type Target = DBConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}