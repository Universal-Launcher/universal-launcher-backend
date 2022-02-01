use crate::database::database::DbPool;
use crate::database::models::modpacks::{Modpack, NewModpack, UpdateModpack};

use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all_modpacks(conn: &DbPool) -> Result<Vec<Modpack>, DbError> {
    use crate::database::schema::modpacks::dsl::*;
    let conn = conn.get()?;

    let m = modpacks.load::<Modpack>(&conn)?;

    Ok(m)
}

pub fn find_modpack_by_id(m_id: i32, conn: &DbPool) -> Result<Option<Modpack>, DbError> {
    use crate::database::schema::modpacks::dsl::*;
    let conn = conn.get()?;

    let modpack = modpacks
        .filter(id.eq(m_id))
        .first::<Modpack>(&conn)
        .optional()?;

    Ok(modpack)
}

pub fn insert_modpack(mo: NewModpack, conn: &DbPool) -> Result<(), DbError> {
    use crate::database::schema::modpacks;
    let conn = conn.get()?;

    diesel::insert_into(modpacks::table)
        .values(&mo)
        .execute(&conn)?;

    Ok(())
}

pub fn update_by_id(m_id: i32, conn: &DbPool, mo: UpdateModpack) -> Result<(), DbError> {
    use crate::database::schema::modpacks::dsl;
    let conn = conn.get()?;

    diesel::update(dsl::modpacks.find(m_id))
        .set(&mo)
        .execute(&conn)?;

    Ok(())
}

pub fn delete_by_id(m_id: i32, conn: &DbPool) -> Result<(), DbError> {
    use crate::database::schema::modpacks::dsl;
    let conn = conn.get()?;

    diesel::delete(dsl::modpacks.find(m_id)).execute(&conn)?;

    Ok(())
}
