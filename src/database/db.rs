use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn init_database() -> DbPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("could not build connection pool")
}
