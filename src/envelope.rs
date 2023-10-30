use anyhow::Result;
use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Default, Clone, Deserialize, Eq, PartialEq)]
pub struct Envelope {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
}

impl Envelope {
    pub fn read_file(filename: &str) -> Result<Envelope> {
        let text = fs::read_to_string(filename)?;
        let envelope: Envelope = toml::from_str(&text).unwrap();

        Ok(envelope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_envelope() {
        let filename = "tests/test-message.toml";
        let envelope = Envelope::read_file(filename);
        println!("env: {:?}", envelope)
    }
}
