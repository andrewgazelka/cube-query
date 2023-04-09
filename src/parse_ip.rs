use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "IP")]
pub struct Ip {
    #[serde(rename = "DBVersion")]
    pub db_version: String,
    #[serde(rename = "IPType")]
    pub ip_type: String,
    pub name: String,
    pub version: String,
    pub about: String,
    #[serde(rename = "RefParameter", default)]
    pub ref_parameters: Vec<RefParameter>,

    #[serde(rename = "GPIO_Pin", default)]
    pub gpio_pins: Vec<GpioPin>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GpioPin {
    pub port_name: String,
    pub name: String,
    #[serde(rename = "SpecificParameter", default)]
    pub specific_parameters: Vec<SpecificParameter>,
    #[serde(rename = "PinSignal", default)]
    pub pin_signals: Vec<PinSignal>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpecificParameter {
    pub name: String,
    #[serde(rename = "PossibleValue", default)]
    pub possible_values: Vec<String>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinSignal {
    pub name: String,
    #[serde(rename = "SpecificParameter", default)]
    pub specific_parameters: Vec<SpecificParameter>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RefParameter {
    pub name: String,
    pub comment: String,
    #[serde(rename = "Type")]
    pub r#type: String,
    pub default_value: Option<String>,
    pub visible: Option<String>,
    #[serde(rename = "PossibleValue", default)]
    pub possible_values: Vec<PossibleValue>,
    #[serde(rename = "Condition", default)]
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PossibleValue {
    pub value: String,
    pub comment: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Condition {
    pub diagnostic: String,
    pub expression: String,
}

pub fn parse_ip() -> anyhow::Result<Ip> {
    let db = crate::paths::obtain_db_folder()?;
    let ip = db.join("IP");
    assert!(ip.exists());
    let gpio = ip.join("GPIO-STM32H747_gpio_v1_0_Modes.xml");
    assert!(gpio.exists());
    let gpio = std::fs::read_to_string(gpio)?;
    let ip: Ip = serde_xml_rs::from_str(&gpio)?;
    Ok(ip)
}

#[cfg(test)]
mod tests {
    use crate::parse_ip::Ip;

    #[test]
    fn test_parse() {
        let db = crate::paths::obtain_db_folder().unwrap();
        let ip = db.join("IP");
        assert!(ip.exists());
        let gpio = ip.join("GPIO-STM32H747_gpio_v1_0_Modes.xml");
        assert!(gpio.exists());
        let gpio = std::fs::read_to_string(gpio).unwrap();
        let ip: Ip = serde_xml_rs::from_str(&gpio).unwrap();

        println!("{:#?}", ip)
    }
}
