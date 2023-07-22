use neopolengine::build_site::build_site;
// use notify::{RecursiveMode, Watcher};
// use notify_debouncer_full::new_debouncer;
use std::fs;
// use std::path::PathBuf;
// use std::time::Duration;
// use std::time::SystemTime;


// The first run deletes the database to ensure
// a fresh start. Then the process watches for
// changes and only updates things that update. 

pub fn main() {
    dbg!("Under construction again");
    let db_path = "/Users/alan/Desktop/neopolengine.sqlite";
    let _ = fs::remove_file(db_path);
    build_site();
    // let path = PathBuf::from("/Users/alan/workshop/alanwsmith.com/content");
    // let _ = watch_files(path);
}

// fn watch_files(path: PathBuf) -> notify::Result<()> {
//     let (tx, rx) = std::sync::mpsc::channel();
//     let mut debouncer = new_debouncer(Duration::from_millis(500), None, tx)?;
//     debouncer.watcher().watch(&path, RecursiveMode::Recursive)?;
//     debouncer.cache().add_root(&path, RecursiveMode::Recursive);
//     let mut last_update = SystemTime::now();
//     let debounce_time = Duration::from_secs(4);
//     for _result in rx {
//         if last_update.elapsed().unwrap() > debounce_time {
//             build_site();
//             last_update = SystemTime::now();
//         }
//     }
//     Ok(())
// }

