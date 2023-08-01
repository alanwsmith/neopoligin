use rusqlite::{Connection, Result};
use std::collections::HashSet;

pub fn get_file_hashes(conn: &Connection) -> Result<HashSet<String>> {
    let mut stmt = conn.prepare("SELECT hash FROM file_hashes")?;
    let rows = stmt.query_map([], |row| row.get(0))?;
    let mut hashes = HashSet::new();
    for hash in rows {
        hashes.insert(hash?);
    }
    Ok(hashes)
}

