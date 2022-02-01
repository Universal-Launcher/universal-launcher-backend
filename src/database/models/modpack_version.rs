use crate::database::schema::modpack_versions;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize, Deserialize, Identifiable)]
#[primary_key(id)]
pub struct ModpackVersion {
    pub id: i32,
    pub modpack_id: i32,
    pub version: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "modpack_versions"]
pub struct NewModpackVersion {
    pub modpack_id: i32,
    pub version: String,
    pub description: Option<String>,
}
