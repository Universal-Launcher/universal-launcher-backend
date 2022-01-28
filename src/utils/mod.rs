use crate::database::database::DbPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct Data {
    pub pool: Arc<DbPool>,
}
