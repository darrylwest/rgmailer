//! rgmailer
//!
//! A simple mailer to send plain text messages through smtp
//!

use anyhow::Result;
use clap::Parser;
use log::{info, warn};
use rgmailer::envelope::Envelope;
use rgmailer::mailer;
use rgmailer::settings::Settings;

#[derive(Debug, Default, Parser)]
#[clap(name = "ngmailer", author, version, about, long_about = None)]
pub struct Config {
    /// set verbose to show log message on the console
    #[clap(short, long, value_parser)]
    pub verbose: bool,

    /// specifiy the envelope toml file with to, from, subject, body and optional process keys
    #[clap(value_parser)]
    pub envelope: String,

    /// specify the application home, defaults to ~/.rgmailer
    #[clap(long, default_value_t = String::from("~/.rgmailer"))]
    pub home: String,

    /// parse the envelope, create the message, login to the smtp server but skip the send
    #[clap(short, long, value_parser)]
    pub dryrun: bool,
}

fn process_request(config: Config, settings: Settings) -> Result<()> {
    info!("process reequest startup with config: {:?}", config);
    let envelope = Envelope::read_file(config.envelope.as_str()).unwrap();
    // process the envelope if necessary
    let message = mailer::prepare_message(envelope);

    let dryrun = config.dryrun;
    if dryrun {
        println!("Woot! dry run success.");
        warn!("this was a dry run");
        Ok(())
    } else {
        mailer::send(settings, message)
    }
}

fn main() -> Result<()> {
    log4rs::init_file("config/console.yaml", Default::default())?;

    info!("logger started.");

    let config = Config::parse();
    println!("cli: {:?}", config);

    let settings = Settings::read(None).expect("settings file not found");

    process_request(config, settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proc_request() {
        let settings = Settings::read(Some(String::from("tests/test-settings.toml"))).unwrap();
        let config = Config {
            home: "home".to_string(),
            envelope: "tests/test-message.toml".to_string(),
            dryrun: true,
            verbose: false,
        };

        let _resp = process_request(config, settings).unwrap();
        // assert!(resp);
    }
}
