use neopolengine::build_site::build_site;
use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;
use std::path::PathBuf;
use std::time::Duration;
use std::time::SystemTime;

pub fn main() {
    let mut path = PathBuf::from("/Users/alan/workshop/alanwsmith.com/templates");
    let _ = watch_files(path);
}

fn watch_files(path: PathBuf) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_millis(500), None, tx)?;
    debouncer.watcher().watch(&path, RecursiveMode::Recursive)?;
    debouncer.cache().add_root(&path, RecursiveMode::Recursive);
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

// fn do_update() {
//     dbg!("Change detected");
// }

// use core::fmt::Error;
// use miette::{IntoDiagnostic, Result};
// use std::time::Duration;
// use watchexec::{
//     action::{Action, Outcome},
//     config::{InitConfig, RuntimeConfig},
//     handler::PrintDebug,
//     Watchexec,
// };
// use watchexec_signals::Signal;

//#[tokio::main]
//async fn main() -> Result<()> {
//    let mut init = InitConfig::default();
//    init.on_error(PrintDebug(std::io::stderr()));
//    let mut runtime = RuntimeConfig::default();
//    runtime.pathset([
//        "/Users/alan/workshop/alanwsmith.com/templates",
//        "/Users/alan/workshop/alanwsmith.com/content",
//    ]);
//    runtime.action_throttle(Duration::new(0, 2000000000));
//    let we = Watchexec::new(init, runtime.clone())?;
//    let mut running = 0;
//    runtime.on_action(move |action: Action| {
//        //
//        async move {
//            // eprintln!("Watchexec Action: {action:?}");
//            let mut stop_running = false;
//            let mut rebuild = false;
//            let mut start_fresh = false;
//            for event in action.events.into_iter() {
//                event.signals().for_each(|sig| match sig {
//                    Signal::Interrupt => {
//                        stop_running = true;
//                    }
//                    _ => {}
//                });
//                if event
//                    .paths()
//                    .any(|(p, _)| p.starts_with("/Users/alan/workshop/alanwsmith.com/templates"))
//                {
//                    rebuild = true;
//                    start_fresh = true;
//                }
//                if event
//                    .paths()
//                    .any(|(p, _)| p.starts_with("/Users/alan/workshop/alanwsmith.com/content"))
//                {
//                    rebuild = true;
//                    // action.outcome(Outcome::Exit);
//                }
//            }
//            if stop_running {
//                action.outcome(Outcome::Exit);
//            } else if start_fresh {
//                action.outcome(Outcome::Exit);
//                println!("TODO: make a full rebuild");
//            } else if rebuild {
//                running += 1;
//                // action.outcome(Outcome::DoNothing);
//                // action.outcome(Outcome::if_running(
//                //     Outcome::DoNothing,
//                //     Outcome::both(Outcome::Clear, Outcome::Start),
//                // ));
//                build_site();
//                dbg!(running);
//            }
//            // action.outcome(Outcome::if_running(
//            //     Outcome::DoNothing,
//            //     Outcome::both(Outcome::Clear, Outcome::Start),
//            // ));
//            Ok::<(), Error>(())
//        }
//    });
//    let _ = we.reconfigure(runtime);
//    let _ = we.main().await.into_diagnostic()?;
//    Ok(())
//}
