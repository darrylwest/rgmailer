use anyhow::Result;
use lettre::transport::smtp::authentication::Credentials;
use log::info;
use serde_derive::Deserialize;
use std::fs;

const SETTINGS: &str = include_str!("../settings.toml");

#[derive(Clone, Debug, Deserialize)]
pub struct Smtp {
    pub host: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub smtp: Smtp,
}

impl Settings {
    pub fn read(filename: Option<String>) -> Result<Settings> {
        let text: String = match filename {
            Some(name) => {
                info!("reading settings file: {}", name);
                fs::read_to_string(name)?
            }
            _ => String::from(SETTINGS),
        };

        let settings: Settings = toml::from_str(&text)?;

        Ok(settings)
    }
}

pub fn parse_creds(settings: Settings) -> Credentials {
    let username = settings.smtp.username;
    let password = settings.smtp.password;

    Credentials::new(username, password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_settings() {
        let settings_file = String::from("tests/test-settings.toml");
        let expected = "should read the settings from: {} settings_file";
        let settings = Settings::read(Some(settings_file)).expect(expected);

        assert_eq!(settings.smtp.host, "smtp.gmail.net");
        assert_eq!(settings.smtp.username, "tester@gmail.com");
        assert_eq!(settings.smtp.password, "mysecretpw");
    }

    #[test]
    fn validate_creds() {
        let settings_file = String::from("tests/test-settings.toml");
        let expected = "should read the settings from: {} settings_file";
        let settings = Settings::read(Some(settings_file)).expect(expected);
        let creds = parse_creds(settings);

        println!("{:?}", creds);
    }
}
