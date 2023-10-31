
use anyhow::Result;
use lettre::{Message, SmtpTransport, Transport};
use crate::settings::{parse_creds, Settings};
use crate::otp::generate_otp;

pub fn prepare_message() -> Result<()> {

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

    // Open a remote connection to gmail
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