use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn init_database() -> Result<DbPool, r2d2::PoolError> {
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);

    Pool::builder().build(manager)
}
