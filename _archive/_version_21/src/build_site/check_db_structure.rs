use crate::build_site::table_exists::table_exists;
use rusqlite::Connection;

pub fn check_db_structure(conn: &Connection) {
    match table_exists(&conn, "file_hashes") {
        Ok(false) => {
            conn.execute("CREATE TABLE file_hashes (hash TEXT)", ())
                .unwrap();
        }
        _ => {}
    }
}

