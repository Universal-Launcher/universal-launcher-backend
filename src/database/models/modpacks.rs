use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::database::schema::modpacks;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize, Identifiable)]
#[primary_key(id)]
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

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[table_name = "modpacks"]
pub struct UpdateModpack {
    pub name: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}
