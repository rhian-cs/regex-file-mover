use std::error::Error;

use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    #[arg(long, default_value_t = String::from("."))]
    directory: String,

    #[arg(long, default_value_t = String::from("Uncategorized"))]
    uncategorized_directory: String,

    #[arg(long)]
    dry_run: bool,

    #[arg(long)]
    wet_run: bool,

    #[arg(short, long)]
    pattern: String,
}

#[derive(Debug)]
pub struct Args {
    pub directory: String,
    pub wet_run: bool,
    pub pattern: String,
    pub uncategorized_directory: String,
}

impl Args {
    pub fn parse() -> Result<Args, Box<dyn Error>> {
        let cli_args = CliArgs::parse();

        validate_args(&cli_args)?;

        Ok(Args {
            directory: cli_args.directory,
            wet_run: cli_args.wet_run,
            pattern: cli_args.pattern,
            uncategorized_directory: cli_args.uncategorized_directory,
        })
    }
}

fn validate_args(cli_args: &CliArgs) -> Result<(), Box<dyn Error>> {
    match (cli_args.wet_run, cli_args.dry_run) {
        (true, true) => Err("--wet-run and --dry-run can't be specified simultaneously".into()),
        (false, false) => Err("--wet-run or --dry-run must be specified".into()),
        _ => Ok(()),
    }
}
