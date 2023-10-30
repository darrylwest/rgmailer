use anyhow::Result;
use lettre::transport::smtp::authentication::Credentials;
use serde_derive::Deserialize;
use std::fs;

// TODO: change to RGMAILER with host, username, pw and return as a tuple
#[derive(Deserialize)]
struct Smtp {
    host: String,
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct Settings {
    smtp: Smtp,
}

impl Settings {
    fn read(filename: &str) -> Result<Settings> {
        let text = fs::read_to_string(filename)?;
        let settings: Settings = toml::from_str(&text)?;

        Ok(settings)
    }
}

pub fn read_creds() -> Credentials {
    let settings = Settings::read("./settings.toml").expect("should read the settings file");

    let host = settings.smtp.host;
    let username = settings.smtp.username;
    let password = settings.smtp.password;

    println!("host: {}", host);

    // TODO: refactor to just return generic username and password
    Credentials::new(username, password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_settings() {
        let settings = Settings::read("tests/test-settings.toml").unwrap();

        assert_eq!(settings.smtp.host, "gmail.smtp.net");
        assert_eq!(settings.smtp.username, "tester@gmail.com");
        assert_eq!(settings.smtp.password, "mysecretpw");
    }
}
