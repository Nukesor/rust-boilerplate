use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(name = "rust-boilerplate", about = "Some about", author, version)]
pub struct CliArguments {
    /// Verbose mode (-v, -vv, -vvv)
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,
}
