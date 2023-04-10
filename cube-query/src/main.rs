use std::process::ExitCode;

use cube_query_lib::clap::Parser;
use cube_query_lib::{run, Args};

fn main() -> ExitCode {
    let args = Args::parse();
    if let Err(e) = run(&args) {
        eprintln!("Error: {}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
