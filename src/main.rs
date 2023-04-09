use std::process::ExitCode;

use clap::Parser;
use prettytable::format::consts::FORMAT_CLEAN;
use prettytable::{row, Attr, Cell, Row, Table};

mod parse_ip;
mod parse_mcu;
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
    let mut db = parse_mcu::parse_xml(&db)?;
    let gpios = parse_ip::parse_ip()?;

    // convert gpios into HashMap name: Vec<PinSignal>
    let gpios = gpios
        .gpio_pins
        .into_iter()
        .map(|pin| (pin.name.to_string(), pin.pin_signals))
        .collect::<std::collections::HashMap<_, _>>();

    db.pins.sort_by_key(|pin| pin.name.to_ascii_lowercase());

    let mut table = Table::new();
    table.set_format(*FORMAT_CLEAN);
    table.add_row(Row::new(vec![
        Cell::new("Pin").with_style(Attr::Bold),
        Cell::new("Use").with_style(Attr::Bold),
        Cell::new("Mode").with_style(Attr::Bold),
    ]));

    for pin in db.pins {
        for signal in pin.signals {
            let signal_name = signal.name.to_ascii_lowercase();
            if signal_name.contains(&filter) {
                let gpio = gpios.get(&pin.name).unwrap();

                let signal = gpio
                    .iter()
                    .find(|signal| signal.name.to_ascii_lowercase() == signal_name)
                    .unwrap();

                let signal_value = signal
                    .specific_parameters
                    .iter()
                    .flat_map(|p| &p.possible_values)
                    .next()
                    .unwrap();
                let (_, signal_value, _) = signal_name_components(&signal_value).unwrap();

                table.add_row(row![pin.name, signal.name, signal_value]);
            }
        }
    }

    table.printstd();

    Ok(())
}

/// name: GPIO_{X}_{Y}
fn signal_name_components(name: &str) -> Option<(&str, &str, &str)> {
    let mut parts = name.split('_');
    let gpio = parts.next()?;
    let x = parts.next()?;
    let y = parts.next()?;
    Some((gpio, x, y))
}
