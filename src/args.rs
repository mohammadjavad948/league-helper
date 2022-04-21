use clap::Parser;

/// a league launch helper
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// config file location; default ~/league_helper.json
    #[clap(short, long)]
    pub config: Option<String>,
}