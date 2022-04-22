use crate::args::Args;
use clap::Parser;
use std::fs;


pub struct Config {

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
    pub fn new() -> Result<Config, &'static str>{
        let location = Config::get_location();

        let file = fs::read_to_string(&location).unwrap();

        // todo: parse config file

        Ok(Config {})
    }
    pub fn check() {
        let location = Config::get_location();

        let file = fs::read_to_string(&location);

        if file.is_err() {
            println!("cannot find the config file, creating a new one.");

            fs::write(
                &location,
                r#"{"launch_helper_location": "", "loop_interval": ""}"#
            );

            std::process::exit(1);
        }
    }
}