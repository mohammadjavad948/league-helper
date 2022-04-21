use crate::args::Args;
use clap::Parser;
use std::fs;


pub struct Config {

}

impl Config {
    pub fn new() -> Result<Config, &'static str>{
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

        let file = fs::read_to_string(&location);

        if file.is_err() {
            println!("cannot find the config file, creating a new one.");

            fs::write(&location, "hello");

            std::process::exit(1);
        }

        let file = file.unwrap();

        // todo: parse config file

        Ok(Config {})
    }
}