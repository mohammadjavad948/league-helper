mod config;
mod args;

extern crate dirs;


fn main() {
    // check the config file
    config::Config::check();

    let conf = config::Config::new();

    println!("{:?}", conf);
}
