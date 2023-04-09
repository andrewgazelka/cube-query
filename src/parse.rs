// static IGNORED_TAGS: &[&[u8]] = &[b"IP"];

use std::fmt::Debug;

use serde::Serialize;

mod raw;

#[derive(Default, Serialize)]
pub struct Mcu {
    pub pins: Vec<Pin>,
}

impl Debug for Mcu {
    /// format in the form
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Mcu");

        for pin in &self.pins {
            s.field(&pin.name, &pin.signals);
        }

        s.finish()
    }
}

#[derive(Default, Serialize)]
pub struct Pin {
    pub name: String,
    pub signals: Vec<raw::Signal>,
}

pub fn parse_xml(s: &str) -> anyhow::Result<Mcu> {
    let mut res = Mcu::default();
    let raw = raw::parse_xml(s)?;

    for event in raw.events {
        if let raw::Event::Pin(pin) = event {
            let signals = pin
                .signals
                .into_iter()
                .filter_map(|signal| match signal {
                    raw::PinEvent::Signal(signal) => Some(signal),
                    _ => None,
                })
                .collect();
            res.pins.push(Pin {
                name: pin.name,
                signals,
            })
        }
    }

    Ok(res)
}
