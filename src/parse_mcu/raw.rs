use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Signal {
    pub name: String,
    #[serde(rename = "IOModes", skip_serializing_if = "Option::is_none")]
    io_modes: Option<String>,
}

impl Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Condition {
    pub diagnostic: String,
    pub expression: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PinEvent {
    Signal(Signal),
    Condition(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Voltage {
    max: f64,
    min: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Current {
    lowest: f64,
    run: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Temperature {
    max: f64,
    min: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ip {
    clock_enable_mode: Option<String>,
    instance_name: String,
    name: String,
    power_domain: Option<String>,
    version: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pin {
    pub name: String,
    pub position: String,
    pub r#type: String, // TODO: make enum
    #[serde(rename = "$value", default)]
    pub signals: Vec<PinEvent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Event {
    Core(String),
    Frequency(u32),
    CCMRam(u32),
    Ram(u32),
    IONb(u32),
    Die(String),
    Flash(u32),
    Voltage(Voltage),
    Current(Current),
    Temperature(Temperature),
    IP(Ip),
    Pin(Pin),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mcu {
    // pub core: String,
    // pub frequency: u32,
    #[serde(rename = "$value")]
    pub events: Vec<Event>,
}

pub fn parse_xml(s: &str) -> anyhow::Result<Mcu> {
    let mcu: Mcu = serde_xml_rs::from_str(s)?;
    Ok(mcu)
}
