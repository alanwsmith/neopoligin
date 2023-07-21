#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::file_lists::file_lists;
// use crate::page::page;
use crate::page::Page;
use crate::sections::sections;
use crate::source_file::title::title;
use crate::source_file::SourceFile;
use minijinja::context;
use minijinja::path_loader;
use minijinja::Environment;
use rusqlite::{Connection, Result};
use sha256::digest;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::vec;
use walkdir::WalkDir;

pub fn build_site() {
    println!("Starting site build");
    let template_root = PathBuf::from("/Users/alan/workshop/alanwsmith.com/templates");
    let content_root = PathBuf::from("/Users/alan/workshop/alanwsmith.com/content");
    let site_root_root = PathBuf::from("/Users/alan/workshop/alanwsmith.com/_site");
    let sqlite_path = "/Users/alan/Desktop/neopolengine.sqlite";
    let mut env = Environment::new();
    let template_path = template_root.display().to_string();
    env.set_loader(path_loader(template_path.as_str()));
    let mut pages: Vec<Page> = vec![];

    println!("Getting file change hash checks");
    let conn = Connection::open(sqlite_path).unwrap();
    check_db_structure(&conn);
    let file_hashes = get_file_hashes(&conn).unwrap();

    println!("Walking content dir: {}", &content_root.display());
    for entry in WalkDir::new(&content_root).into_iter() {
        let initial_path = entry.as_ref().unwrap().path().to_path_buf();
        if let Some(ext) = initial_path.extension() {
            if ext == "neo" {
                // Process neo files
                let source_string = fs::read_to_string(&initial_path).unwrap();
                let source_hash = digest(source_string.as_str());
                if !file_hashes.contains(&source_hash) {
                    let mut p = Page::new_from(&source_string);
                    let mut page_path = PathBuf::from("/");
                    p.source_hash = Some(source_hash);
                    page_path.push(initial_path.strip_prefix(&content_root).unwrap());
                    page_path.set_extension("html");
                    p.path = Some(page_path);
                    pages.push(p);
                }
            } else {
                // Move everything else over directly
                let mut output_path = site_root_root.clone();
                let file_sub_path = initial_path.strip_prefix(&content_root).unwrap();
                output_path.push(file_sub_path);
                let parent_dir = output_path.parent().unwrap();
                if initial_path.to_path_buf().is_file() {
                    if !parent_dir.exists() {
                        fs::create_dir_all(parent_dir).unwrap();
                    }
                    fs::copy(initial_path, output_path).unwrap();
                }
            }
        }
    }

    // add or remove `.take(7)`` behind `.iter()`` for testing
    pages.iter().for_each(|page| {
        println!("::Making:: {}\n", page.path.as_ref().unwrap().display());
        let template_id = match (&page.r#type, &page.template) {
            (None, None) => "post",
            _ => "post",
        };
        let template = env.get_template(format!("{}/index.html", template_id,).as_str());
        let output = template
            .unwrap()
            .render(context!(
                page => page
            ))
            .unwrap();
        // The `page.path`` has an absolute path from the site
        // root. If you try to add that directly on top of the
        // site_root_root, it replaces it instead.
        // https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html#method.push
        let mut file_path = site_root_root.clone();
        let relative_site_path = page.path.as_ref().unwrap().strip_prefix("/").unwrap();
        file_path.push(relative_site_path);

        let dir_path = file_path.parent().unwrap();
        fs::create_dir_all(dir_path).unwrap();
        fs::write(file_path, output).unwrap();
        insert_hash(&conn, page.source_hash.as_ref().unwrap().as_str()).unwrap();
    });
    println!("Process complete");
}

fn insert_hash(conn: &Connection, hash: &str) -> Result<()> {
    let mut stmt = conn.prepare("INSERT INTO file_hashes (hash) VALUES (?1)")?;
    stmt.execute([hash])?;
    Ok(())
}

fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
    let rows = stmt.query_map([table_name], |_| Ok(()))?;
    if rows.count() == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn check_db_structure(conn: &Connection) {
    match table_exists(&conn, "file_hashes") {
        Ok(false) => {
            conn.execute("CREATE TABLE file_hashes (hash TEXT)", ())
                .unwrap();
        }
        _ => {}
    }
}

fn get_file_hashes(conn: &Connection) -> Result<HashSet<String>> {
    let mut stmt = conn.prepare("SELECT hash FROM file_hashes")?;
    let rows = stmt.query_map([], |row| row.get(0))?;
    let mut hashes = HashSet::new();
    for hash in rows {
        hashes.insert(hash?);
    }
    Ok(hashes)
}
