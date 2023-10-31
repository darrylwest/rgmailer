use anyhow::Result;
use rgmailer::mailer;
use rgmailer::settings::Settings;
use rgmailer::envelope::Envelope;
use std::env;

#[derive(Clone, Debug, PartialEq)]
struct Config {
    home: String,
    envelope_file: String,
    dryrun: bool,
}

impl Config {
    fn parse_cli() -> Result<Config> {
        // TODO: move this to cli.rs?
        let argc = env::args().len();
        if argc != 2 {
            eprintln!("Error: Use: rgmailer file");
            eprintln!("args: {}", argc);
            panic!("must supply an envelope file");
        }

        // simulate get these from the command line
        let dryrun = false;
        let mut args = env::args();
        let filename = args.nth(1).unwrap();

        let config = Config{
            home: "home".to_string(),
            envelope_file: filename,
            dryrun: dryrun,
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
    process_request(Config::parse_cli().unwrap())
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
}
