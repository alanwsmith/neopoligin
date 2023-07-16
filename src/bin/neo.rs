use core::fmt::Error;
use miette::{IntoDiagnostic, Result};
use neopolengine::build_site::build_site;
use std::time::Duration;
use watchexec::{
    action::{Action, Outcome},
    config::{InitConfig, RuntimeConfig},
    handler::PrintDebug,
    Watchexec,
};
use watchexec_signals::Signal;

#[tokio::main]
async fn main() -> Result<()> {
    let mut init = InitConfig::default();
    init.on_error(PrintDebug(std::io::stderr()));
    let mut runtime = RuntimeConfig::default();
    runtime.pathset([
        "/Users/alan/workshop/alanwsmith.com/templates",
        "/Users/alan/workshop/alanwsmith.com/content",
    ]);
    runtime.action_throttle(Duration::new(0, 100000000));
    let we = Watchexec::new(init, runtime.clone())?;
    runtime.on_action(move |action: Action| async move {
        let mut stop_running = false;
        let mut rebuild = false;
        let mut start_fresh = false;
        for event in action.events.iter() {
            event.signals().for_each(|sig| match sig {
                Signal::Interrupt => {
                    stop_running = true;
                }
                _ => {}
            });
            if event
                .paths()
                .any(|(p, _)| p.starts_with("/Users/alan/workshop/alanwsmith.com/templates"))
            {
                rebuild = true;
                start_fresh = true;
            }
            if event
                .paths()
                .any(|(p, _)| p.starts_with("/Users/alan/workshop/alanwsmith.com/content"))
            {
                rebuild = true;
            }
        }
        if stop_running {
            action.outcome(Outcome::Exit);
        }
        if start_fresh {
            println!("TODO: make a full rebuild");
        }
        if rebuild {
            build_site();
        }
        Ok::<(), Error>(())
    });
    let _ = we.reconfigure(runtime);
    let _ = we.main().await.into_diagnostic()?;
    Ok(())
}
