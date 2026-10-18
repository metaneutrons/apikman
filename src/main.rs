//! Human-facing CLI foundation. This build does not read or store credentials.

use std::collections::BTreeMap;
use std::io::{self, Write};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use serde::Serialize;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "API credential management (development scaffold).\nVault storage, native unlock, cloud synchronization and the AI broker are not implemented."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Report implemented capabilities without opening a vault or key store.
    Status {
        /// Emit a machine-readable capability report.
        #[arg(long)]
        json: bool,
    },
}

#[derive(Serialize)]
struct Status {
    schema_version: u8,
    version: &'static str,
    stage: &'static str,
    #[serde(flatten)]
    capabilities: BTreeMap<&'static str, bool>,
}

impl Status {
    fn current() -> Self {
        Self {
            schema_version: 1,
            version: env!("CARGO_PKG_VERSION"),
            stage: "development-scaffold",
            capabilities: [
                "vault_storage",
                "native_unlock",
                "icloud_unlock",
                "cloud_sync",
                "ai_broker",
            ]
            .into_iter()
            .map(|capability| (capability, false))
            .collect(),
        }
    }
}

fn run(cli: &Cli, output: &mut impl Write) -> io::Result<()> {
    match cli.command {
        Command::Status { json: true } => {
            serde_json::to_writer(&mut *output, &Status::current())?;
            writeln!(output)
        }
        Command::Status { json: false } => writeln!(
            output,
            "apikman {}: development scaffold\nVault storage, native unlock, iCloud unlock, cloud sync and the AI broker are not implemented.",
            env!("CARGO_PKG_VERSION")
        ),
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli, &mut io::stdout().lock()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(_) => {
            eprintln!("apikman: could not write output");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_failure_is_returned_to_the_caller() {
        let cli = Cli::try_parse_from(["apikman", "status", "--json"]).unwrap();
        let error = run(&cli, &mut io::Cursor::new(&mut [0_u8; 1][..])).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::WriteZero);
    }
}
