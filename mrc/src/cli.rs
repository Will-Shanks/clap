use std::path::PathBuf;
use clap::Parser;
use clap::Subcommand;


#[derive(Parser)]
#[clap(disable_help_subcommand = true)]
pub struct Opt {
    #[clap(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Check {
        file: PathBuf,
    },
    Manpages {
        dir: PathBuf,
    },
    #[clap(subcommand)]
    Temp(TempCommand),
}

#[derive(Subcommand)]
enum TempCommand {
    TestA,
    TestB,
}
