use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "rshell",
    bin_name = "rshell",
    about = "Rashing's Shell",
    version,
    arg_required_else_help = true,
    disable_help_subcommand = true,
    disable_version_flag = true
)]
struct Cli {
    #[command(subcommand)]
    command: Subcommands,

    #[arg(short, long, help = "Print version", action = ArgAction::Version)]
    version: ()
}

#[derive(Subcommand)]
enum Subcommands {
    #[command(about = "Run a config file")]
    Run {
        #[arg(
            short,
            long,
            value_name = "PATH",
            help = "Override default config file path"
        )]
        config: Option<PathBuf>
    }
}

fn main() {
    let cli = Cli::parse();
    todo!("handle cli")
}
