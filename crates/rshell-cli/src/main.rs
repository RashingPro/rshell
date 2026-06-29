use clap::{ArgAction, Parser, Subcommand};
use log::{LevelFilter, error};
use rshell_runtime::Runtime;
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
    version: (),

    #[arg(
        short,
        long,
        action,
        help = "Enable more verbose logging",
        global = true
    )]
    debug: bool
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

    colog::default_builder()
        .filter_level(if cli.debug {
            LevelFilter::max()
        } else {
            LevelFilter::Info
        })
        .init();

    match cli.command {
        Subcommands::Run { config } => {
            run(config.unwrap_or(PathBuf::from(env!("DEFAULT_CONFIG_PATH"))))
        }
        _ => unimplemented!("unimplemented subcommand")
    }
}

#[tokio::main]
async fn run(config: PathBuf) {
    let runtime = Runtime::new(config);
    if let Err(err) = runtime.run().await {
        error!("{}", err);
    }
}
