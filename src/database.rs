use anyhow::Result;

use crate::serenity;

const DATABASE_FILE_NAME: &str = "db.sqlite";

pub fn init() -> Result<()> {
    let connection = rusqlite::Connection::open(DATABASE_FILE_NAME)?;

    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            kennung TEXT NOT NULL
        );",
        [],
    )?;

    Ok(())
}

pub fn is_verified(user_id: serenity::UserId) -> Result<bool> {
    let connection = rusqlite::Connection::open(DATABASE_FILE_NAME)?;

    let mut stmt =
        connection.prepare_cached("SELECT (COUNT(*) > 0) AS found FROM users WHERE id == ?1")?;

    let mut rows = stmt.query(rusqlite::params![user_id.0])?;

    Ok(rows
        .next()?
        .and_then(|r| r.get("found").ok())
        .unwrap_or_default())
}

pub fn add_verified(user_id: serenity::UserId, tum_id: &str) -> Result<()> {
    let connection = rusqlite::Connection::open(DATABASE_FILE_NAME)?;

    let mut stmt = connection.prepare_cached("INSERT INTO users(id, kennung) VALUES(?, ?)")?;

    stmt.execute(rusqlite::params![user_id.0, tum_id])?;

    Ok(())
}

pub fn get_verifications(tum_id: &str) -> Result<usize> {
    let connection = rusqlite::Connection::open(DATABASE_FILE_NAME)?;

    let mut stmt =
        connection.prepare_cached("SELECT COUNT(*) AS count FROM users WHERE kennung == ?")?;

    let mut rows = stmt.query(rusqlite::params![tum_id])?;

    Ok(rows
        .next()?
        .and_then(|r| r.get("count").ok())
        .unwrap_or_default())
}
