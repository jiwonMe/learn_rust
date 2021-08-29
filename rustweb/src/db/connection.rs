use std::ops::Deref;

use r2d2;
use r2d2::PooledConnection;
use diesel::mysql::MysqlConnection;
use diesel::mysql::*;
use r2d2_diesel::ConnectionManager;

use dotenv::dotenv;
use rocket::https::Status;
use rocket::request::{ self, FromRequest };
use rocket::{ Outcome, Request, Status };
use std::env;
pub type Pool = r2d2::Pool>;

pub fn init_pool() -> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::::new(database_url);
    Pool::new(manager).expect("db pool")
}

pub struct Conn(pub PooledConnection>);

impl Deref for Conn {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome {
        let pool = request.guard::>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}