use rusqlite::{Connection, Result};

pub fn insert_hash(conn: &Connection, hash: &str) -> Result<()> {
    let mut stmt = conn.prepare("INSERT INTO file_hashes (hash) VALUES (?1)")?;
    stmt.execute([hash])?;
    Ok(())
}