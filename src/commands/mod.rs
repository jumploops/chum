pub mod archive;
pub mod check;
pub mod init;
pub mod swim;

use crate::cli::{Cli, Command};
use anyhow::Result;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Init(args) => init::run(args),
        Command::Check(args) => {
            let json = args.json;
            let report = check::run_report(args)?;
            if json {
                check::print_report_json(&report)?;
            } else {
                check::print_report(&report);
            }
            if report.has_errors() {
                std::process::exit(1);
            }
            Ok(())
        }
        Command::Archive(args) => archive::run(args),
        Command::Swim(args) => swim::run(args),
    }
}
