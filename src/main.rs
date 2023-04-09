use std::process::ExitCode;

use clap::Parser;

mod parse;
mod paths;

#[derive(Debug, Parser)]
struct Args {
    chip: String,
    filter: Option<String>,
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run() -> anyhow::Result<()> {
    let args = Args::parse();
    let filter = args.filter.unwrap_or_default().to_ascii_lowercase();
    let db = paths::obtain_db(&args.chip)?;
    let db = std::fs::read_to_string(db)?;
    let db = parse::parse_xml(&db)?;

    for pin in db.pins {
        for signal in pin.signals {
            let name = signal.name.to_ascii_lowercase();
            if name.contains(&filter) {
                println!("{}: {}", pin.name, signal.name);
            }
        }
    }

    Ok(())
}
