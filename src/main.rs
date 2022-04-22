mod config;
mod args;

extern crate dirs;


fn main() {
    // check the config file
    config::Config::check();

    config::Config::new();
}
