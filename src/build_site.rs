use crate::build_site::check_db_structure::check_db_structure;
// use crate::build_site::get_file_hashes::get_file_hashes;
use crate::build_site::insert_hash::insert_hash;
use crate::page::Page;
use crate::universe::Universe;
use minijinja::context;
use minijinja::path_loader;
use minijinja::value::Value;
use minijinja::Environment;
use rusqlite::Connection;
use sha256::digest;
use std::fs;
use std::path::PathBuf;
use std::vec;
use walkdir::WalkDir;

pub mod check_db_structure;
pub mod get_file_hashes;
pub mod insert_hash;
pub mod table_exists;

pub fn make_full_link_list(e: &Environment, u: Universe) {
    let uvx = Value::from_struct_object(u.clone());
    let tmpl = e.get_template("_src/templates/full_link_list.html");
    let rv = tmpl.expect("Boom").render(context!(uvx => uvx)).unwrap();
    let _ = fs::write(
        "/Users/alan/workshop/alanwsmith.com/templates/_src/includes/full_link_list.html",
        rv,
    );
}

pub fn build_site() {
    println!("Starting site build");
    let template_root = PathBuf::from("/Users/alan/workshop/alanwsmith.com/templates");
    let content_root = PathBuf::from("/Users/alan/workshop/alanwsmith.com/content");
    let site_root_root = PathBuf::from("/Users/alan/workshop/alanwsmith.com/_site");
    let sqlite_path = "/Users/alan/Desktop/neopolengine.sqlite";
    let mut env = Environment::new();
    let template_path = template_root.display().to_string();
    env.set_loader(path_loader(template_path.as_str()));
    let mut u = Universe {
        pages: vec![],
        cache_full_link_list: None,
    };
    println!("Getting file change hash checks");
    let conn = Connection::open(sqlite_path).unwrap();
    check_db_structure(&conn);
    // let file_hashes = get_file_hashes(&conn).unwrap();
    println!("Walking content dir: {}", &content_root.display());
    for entry in WalkDir::new(&content_root).into_iter() {
        let initial_path = entry.as_ref().unwrap().path().to_path_buf();
        if let Some(ext) = initial_path.extension() {
            if ext == "neo" {
                // Process neo files
                let source_string = fs::read_to_string(&initial_path).unwrap();
                let source_hash = digest(source_string.as_str());
                // this is the original way to do things that only
                // outputs changed files but that's off right now
                // untill the links can all be generated.
                // if !file_hashes.contains(&source_hash) {

                if 1 == 1 {
                    let mut p = Page::new_from(&source_string);
                    let mut page_path = PathBuf::from("/");
                    p.source_hash = Some(source_hash);
                    page_path.push(initial_path.strip_prefix(&content_root).unwrap());
                    page_path.set_extension("html");
                    p.path = Some(page_path);
                    dbg!(&p.path);
                    p.load_image_paths();
                    match p.title_string() {
                        Some(_) => u.pages.push(p),
                        None => {}
                    }
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
    let mut counter = 0;
    let uv = Value::from_struct_object(u.clone());
    // TODO: Turn this on when you're done testing
    make_full_link_list(&env, u.clone());
    // add or remove `.take(7)`` behind `.into_iter()`` for testing
    u.pages.clone().into_iter().for_each(|page| {
        // u.pages.clone().into_iter().take(1).for_each(|page| {
        counter += 1;
        // dbg!(&counter);
        // println!("::Making:: {}\n", page.path.as_ref().unwrap().display());
        // TODO: Get the post template here
        //let template_id = "post".to_string();
        let template_id = match page.clone().page_type() {
            Some(x) => x,
            None => "post".to_string(),
        };
        //dbg!(&template_id);
        //let template_id = page.page_type();
        let tmpl = env.get_template(format!("{}/index.html", template_id,).as_str());
        let pg = Value::from_struct_object(page.clone());
        let rv = tmpl
            .expect("Boom")
            .render(context!(pg => pg, uv => uv))
            .unwrap();
        let mut file_path = site_root_root.clone();
        let relative_site_path = page.path.as_ref().unwrap().strip_prefix("/").unwrap();
        file_path.push(relative_site_path);
        let dir_path = file_path.parent().unwrap();
        fs::create_dir_all(dir_path).unwrap();
        fs::write(file_path, rv).unwrap();
        insert_hash(&conn, page.source_hash.as_ref().unwrap().as_str()).unwrap();
    });
    println!("Process complete");
}
