use crate::args::Args;
use clap::Parser;
use serde::{Deserialize};
use std::fs;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub launch_helper_location: String,
    pub loop_interval_ms: u32
}

impl Config {
    pub fn get_location() -> String {
        let args = Args::parse();

        let mut location = String::new();

        if args.config.is_none() {
            location = format!(
                "{}/league_helper.json",
                dirs::home_dir().unwrap().to_str().unwrap()
            );
        } else {
            location = args.config.unwrap();
        }

        location
    }
    pub fn new() -> Config {
        let location = Config::get_location();

        let file = fs::read_to_string(&location).unwrap();

        let deserialize: Result<Config, serde_json::Error> = serde_json::from_str(&*file);

        if deserialize.is_err() {
            println!("cant deserialize the config file. try deleting it and run the app again");

            std::process::exit(1);
        }

        deserialize.unwrap()
    }
    pub fn check() {
        let location = Config::get_location();

        let file = fs::read_to_string(&location);

        if file.is_err() {
            println!("cannot find the config file, creating a new one.");

            fs::write(
                &location,
                r#"{"launch_helper_location": "", "loop_interval_ms": 1000}"#
            );

            std::process::exit(1);
        }
    }
}