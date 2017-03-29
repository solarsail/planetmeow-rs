// DB ORM
use diesel::pg::PgConnection;

// Connection pool
use r2d2::{ Pool, Config, PooledConnection, GetTimeout };
use r2d2_diesel::ConnectionManager;

// Environment
use dotenv::dotenv;
use std::env;

// Provides DB access for Rocket
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;
use rocket::Request;


pub mod post;
pub mod visitor;
pub mod comment;


#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    RecordNotFound,
    ForeignKeyViolation,
    UniqueViolation,
    UnableToSendCommand,
    DatabaseError,
}


pub type DBResult<T> = Result<T, Error>;


pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;

    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
}


lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
}


fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let config = Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(config, manager).expect("Failed to create pool.")
}

