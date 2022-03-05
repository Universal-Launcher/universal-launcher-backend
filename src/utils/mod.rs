use crate::database::{self, database::DbPool};

pub mod errors;
pub mod hash;
#[derive(Clone)]
pub struct Data {
    pub pool: Box<DbPool>,
    pub hash: Box<hash::Hash>,
}

pub fn load_data() -> Result<Data, String> {
    let db_pool =
        Box::new(database::database::init_database().map_err(|_| "Failed to init database")?);

    let data = Data {
        pool: db_pool,
        hash: Box::new(hash::Hash::new()),
    };

    Ok(data)
}
