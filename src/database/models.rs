use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::schema::modpacks;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Modpack {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub enabled: bool,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "modpacks"]
pub struct NewModpack {
    pub name: String,
    pub description: Option<String>,
}
