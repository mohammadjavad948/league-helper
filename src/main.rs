mod config;
mod args;

extern crate dirs;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // check the config file
    config::Config::check();

    let (tx, rx) = mpsc::channel();

    let observer_thread = thread::spawn(move || {
        let config = config::Config::new();

        loop {

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
