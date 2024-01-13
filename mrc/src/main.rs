mod cli;
use cli::Opt;
use clap::Parser;
fn main() {
    let _args = Opt::parse();
    println!("hello world");
}
