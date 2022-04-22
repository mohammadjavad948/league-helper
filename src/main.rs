mod config;
mod args;

extern crate dirs;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use sysinfo::{System, SystemExt, ProcessExt};

fn main() {
    // check the config file
    config::Config::check();

    let (tx, rx) = mpsc::channel();

    let observer_thread = thread::spawn(move || {
        let config = config::Config::new();
        let mut system_info = System::new_all();

        loop {

            system_info.refresh_all();

            let game= system_info.processes_by_exact_name("");

            if game.count() > 0 {
                // todo: launch league helper;
            }

            thread::sleep(
                Duration::from_millis(
                    config.loop_interval_ms
                )
            );
        }
    });

    let conf = config::Config::new();

    observer_thread.join();
}
