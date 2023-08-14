use neopolengine::build_site::build_site;
use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use std::time::SystemTime;

// The first run deletes the database to ensure
// a fresh start. Then the process watches for
// changes and only updates things that update.

pub fn main() {
    try_to_set_tmux_title();
    dbg!("Under construction again");
    let db_path = "/Users/alan/Desktop/neopolengine.sqlite";
    let _ = fs::remove_file(db_path);
    build_site();
    let path = PathBuf::from("/Users/alan/workshop/alanwsmith.com/content");
    let _ = watch_files(path);
}

fn watch_files(path: PathBuf) -> notify::Result<()> {
    // This is a hack, this direcotory should be passed
    // via a vec
    let template_path = PathBuf::from("/Users/alan/workshop/alanwsmith.com/templates");
    let (tx, rx) = std::sync::mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_millis(1000), None, tx)?;
    debouncer.watcher().watch(&path, RecursiveMode::Recursive)?;
    debouncer.cache().add_root(&path, RecursiveMode::Recursive);
    debouncer
        .watcher()
        .watch(&template_path, RecursiveMode::Recursive)?;
    debouncer
        .cache()
        .add_root(&template_path, RecursiveMode::Recursive);
    let mut last_update = SystemTime::now();
    let debounce_time = Duration::from_secs(4);
    for _result in rx {
        if last_update.elapsed().unwrap() > debounce_time {
            build_site();
            last_update = SystemTime::now();
        }
    }
    Ok(())
}

pub fn try_to_set_tmux_title() {
    let args: Vec<&str> = vec!["select-pane", "-T", "Neopoligin"];
    let _ = Command::new("tmux").args(args).output().unwrap();
}
