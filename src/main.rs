use anyhow::Result;
use rgmailer::envelope::Envelope;
use rgmailer::mailer;
use rgmailer::settings::Settings;
use std::env;

#[derive(Clone, Debug, PartialEq)]
struct Config {
    home: String,
    envelope_file: String,
    dryrun: bool,
}

impl Config {
    fn parse_cli(args: Vec<String>) -> Result<Config> {
        // simulate get these from the command line
        let drun = false;
        let filename = args[1].to_string();

        let config = Config {
            home: "home".to_string(),
            envelope_file: filename,
            dryrun: drun,
        };

        Ok(config)
    }
}

fn process_request(config: Config) -> Result<()> {
    let envelope = Envelope::read_file(config.envelope_file.as_str()).unwrap();
    // process the envelope if necessary
    let message = mailer::prepare_message(envelope);
    let settings = Settings::read(None).expect("settings file not found");

    if !config.dryrun {
        return mailer::send(settings, message);
    }

    Ok(())
}

fn main() -> Result<()> {
    // TODO: move this to cli.rs?k
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: Use: rgmailer file");
        eprintln!("args: {:?}", args);
        panic!("must supply an envelope file");
    }

    let config = Config::parse_cli(env::args().collect()).unwrap();
    process_request(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proc_request() {
        let config = Config {
            home: "home".to_string(),
            envelope_file: "tests/test-message.toml".to_string(),
            dryrun: true,
        };

        let _resp = process_request(config).unwrap();
        // assert!(resp);
    }

    #[test]
    fn parce_cli() {
        let args = [
            "rgmailer".to_string(),
            "tests/test-message.toml".to_string(),
            "--dryrun".to_string(),
        ];

        let config = Config::parse_cli(args.to_vec()).unwrap();
        assert!(config.dryrun == false);
    }
}
