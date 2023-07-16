use core::fmt::Error;
use miette::Result;
use std::path::PathBuf;
use std::time::Duration;
use watchexec::action::Action;
use watchexec::action::Outcome;
use watchexec::config::InitConfig;
use watchexec::config::RuntimeConfig;
use watchexec::Watchexec;
use watchexec_signals::Signal;

struct Widget {}

impl Widget {
    pub fn update(&mut self, path: PathBuf) {
        dbg!(path);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting process");
    let mut widget = Widget {};
    let init = InitConfig::default();
    let mut runtime = RuntimeConfig::default();
    runtime.pathset([
        "/Users/alan/workshop/alanwsmith.com/templates",
        "/Users/alan/Grimoire",
        "/Users/alan/workshop/neopolengine/target/release",
    ]);
    runtime.action_throttle(Duration::new(0, 3000000000));
    runtime.on_action(move |action: Action| {
        let mut stop: bool = false;
        let mut paths: Vec<PathBuf> = vec![];
        action.events.iter().for_each(|event| {
            event.signals().for_each(|sig| match sig {
                Signal::Interrupt => {
                    println!("Caught Interrupt: Stopping");
                    stop = true;
                }
                _ => {}
            });
            event
                .paths()
                .for_each(|path| paths.push(path.0.to_path_buf()));
        });
        if stop {
            action.outcome(Outcome::Exit);
        }
        paths.dedup();
        paths
            .iter()
            .filter(|p| is_visible(p.to_path_buf()))
            .for_each(|path| {
                widget.update(path.to_path_buf());
            });
        async move { Ok::<(), Error>(()) }
    });
    let we = Watchexec::new(init, runtime)?;
    we.main().await.unwrap().unwrap();
    Ok(())
}

fn is_visible(entry: PathBuf) -> bool {
    entry
        .file_name()
        .unwrap()
        .to_str()
        .map(|s| !s.starts_with(".") && !s.ends_with("~"))
        .unwrap_or(false)
}
