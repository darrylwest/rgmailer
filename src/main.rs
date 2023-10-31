use anyhow::Result;
use lettre::{Message, SmtpTransport, Transport};
use rgmailer::otp::generate_otp;
use rgmailer::settings::{parse_creds, Settings};
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

    // TODO: read to, from, subject and body from toml file
    let from = "darryl.west<darryl.west@raincitysoftware.com>";
    let to = "<dpw500@raincitysoftware.com>";

    let subject = "otp";
    let otp = generate_otp();
    let body = format!("{}", otp);

    println!("otp: {} to: {}", otp, to);

    let email = Message::builder()
        .from(from.parse().unwrap())
        .reply_to(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    let settings =
        Settings::read(None).expect("should have read the settings file; is it missing?");
    let creds = parse_creds(settings.clone());

    // Open a remote connection to gmail:w
    let host = settings.clone().smtp.host;
    let mailer = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }

    Ok(())
}
