use anyhow::Result;
use rgmailer::mailer;
use rgmailer::settings::Settings;
use rgmailer::envelope::Envelope;
use std::env;

fn main() -> Result<()> {
    // TODO: move this to cli.rs
    let argc = env::args().len();
    if argc != 2 {
        eprintln!("Error: Use: rgmailer file");
        eprintln!("args: {}", argc);
        panic!("must supply an envelope file");
    }

    let mut args = env::args();
    let filename = args.nth(1).unwrap();

    println!("filename is {}", filename);
    let envelope = Envelope::read_file(filename.as_str()).unwrap();
    let settings = Settings::read(None).expect("settings file not found");

    let message = mailer::prepare_message(envelope);

    mailer::send(settings, message)
}
