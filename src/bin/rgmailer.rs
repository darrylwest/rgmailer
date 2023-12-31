//! rgmailer
//!
//! A simple mailer to send plain text messages through smtp
//!

use anyhow::Result;
use clap::Parser;
use log::{error, info, warn};
use rgmailer::envelope::Envelope;
use rgmailer::fileops;
use rgmailer::mailer;
use rgmailer::settings::Settings;
use std::env;

#[derive(Clone, Debug, Default, Parser)]
#[clap(name = "ngmailer", author, version, about, long_about = None)]
pub struct Config {
    /// set verbose to show log message on the console
    #[clap(short, long, value_parser)]
    pub verbose: bool,

    /// specifiy the envelope toml file with to, from, subject, body and optional process keys
    #[clap(value_parser)]
    pub envelope: String,

    /// specify the application home, defaults to ~/.rgmailer
    #[clap(long, default_value_t = String::from(".rgmailer"))]
    pub home: String,

    /// parse the envelope, create the message, login to the smtp server but skip the send
    #[clap(short, long, value_parser)]
    pub dryrun: bool,
}

fn process_request(config: Config, settings: Settings) -> Result<()> {
    info!("process request startup with config: {:?}", config);
    let filename = config.envelope.as_str();
    let envelope = match Envelope::read_file(filename) {
        Ok(envelope) => envelope,
        Err(e) => {
            let msg = format!("Error reading envelope from: {} {}", filename, e);
            eprintln!("\nERROR! {}\n", msg);
            error!("{}", msg);
            return Err(e);
        }
    };

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

fn configure_and_send(config: Config) -> Result<()> {
    let mut log_config_file = "config/rolling.yaml";

    if config.verbose {
        info!("config the console logger");
        log_config_file = "config/console-rolling.yaml";
    }

    // read and embed the config; run-time write to the config folder then init the logger
    // let config_str = include_str!("../config/console.yaml");
    // println!("{}", config_str);
    // TODO check for the logs folder; if not found, then start the console
    let banner = "********************************************";
    match log4rs::init_file(log_config_file, Default::default()) {
        Ok(_) => info!("{} logger started {}", banner, banner),
        Err(e) => eprintln!("error starting logger: {:?}", e),
    }

    match Settings::read(None) {
        Ok(settings) => process_request(config, settings),
        Err(e) => Err(e),
    }
}

fn run(config: Config) -> Result<()> {
    match configure_and_send(config.clone()) {
        Ok(_) => {
            // now check for structure and mv from queue to sent
            fileops::move_to_sent(&config.envelope)
        }
        Err(e) => {
            let msg = format!("{}", e);
            eprintln!("\nError! {}\n", msg);
            error!("{e}");
            Err(e)
        }
    }
}

fn main() -> Result<()> {
    env::set_current_dir(fileops::app_home())?;
    run(Config::parse())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let config = Config {
            home: "home".to_string(),
            envelope: "tests/test-message.toml".to_string(),
            dryrun: true,
            verbose: false,
        };

        let resp = run(config);
        println!("{:?}", resp);
    }

    #[test]
    fn test_bad_run() {
        let config = Config {
            home: "home".to_string(),
            envelope: "tests/no-file.toml".to_string(),
            dryrun: false,
            verbose: false,
        };

        let resp = run(config);
        println!("{:?}", resp.err());
    }

    #[test]
    fn test_configure_and_send() {
        let config = Config {
            home: "home".to_string(),
            envelope: "tests/test-message.toml".to_string(),
            dryrun: true,
            verbose: true,
        };

        let resp = configure_and_send(config);
        println!("{:?}", resp);
    }

    #[test]
    fn proc_request() {
        let settings = Settings::read(Some(String::from("tests/test-settings.toml"))).unwrap();
        let config = Config {
            home: "home".to_string(),
            envelope: "tests/test-message.toml".to_string(),
            dryrun: false,
            verbose: false,
        };

        let resp = process_request(config, settings);
        println!("{:?}", resp.err());
    }

    #[test]
    fn proc_request_dryrun() {
        let settings = Settings::read(Some(String::from("tests/test-settings.toml"))).unwrap();
        let config = Config {
            home: "home".to_string(),
            envelope: "tests/test-message.toml".to_string(),
            dryrun: true,
            verbose: false,
        };

        let resp = process_request(config, settings);
        println!("{:?}", resp);
        resp.expect("should do a dry run");
    }
}
