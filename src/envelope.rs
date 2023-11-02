use anyhow::Result;
use log::{error, info};
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
        let text = match fs::read_to_string(filename) {
            Ok(text) => text,
            Err(e) => return Err(e.into()),
        };

        match toml::from_str(&text) {
            Ok(envelope) => {
                info!("envelope read and parsed from: {}", filename);
                Ok(envelope)
            }
            Err(e) => {
                let msg = format!("Error reading/parsing envelope from: {} {}", filename, e);
                eprintln!("\nERROR! {}\n", msg);
                error!("{}", msg);
                Err(e.into())
            }
        }
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

    #[test]
    fn read_file_missing() {
        let filename = "tests/nothing.toml";
        let resp = Envelope::read_file(filename);
        let err = resp.err().unwrap();
        println!("resp: {:?}", err);
        assert_eq!(err.to_string(), "No such file or directory (os error 2)");
    }

    #[test]
    fn read_bad_parse() {
        let filename = "tests/bad-envelope.toml";
        let resp = Envelope::read_file(filename);
        let err = resp.err().unwrap();
        println!("resp: {:?}", err);
    }
}
