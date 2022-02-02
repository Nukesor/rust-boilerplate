use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "rust-boilerplate", about = "Some about", author, version)]
pub struct CliArguments {
    /// Verbose mode (-v, -vv, -vvv)
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: u8,
}
