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
    #[command(name = "check-me")]
    Check {
        file: PathBuf,
    },
    Manpages {
        dir: PathBuf,
    },
    #[clap(subcommand)]
    Temp(TempCommand),
    #[clap(subcommand)]
    #[command(name = "other-temp")]
    TempOther(TempCommand),
}

#[derive(Subcommand)]
enum TempCommand {
    TestA,
    TestB,
}
