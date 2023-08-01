use rusqlite::{Connection, Result};

pub fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
    let rows = stmt.query_map([table_name], |_| Ok(()))?;
    if rows.count() == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}