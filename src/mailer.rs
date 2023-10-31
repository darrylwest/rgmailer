

use anyhow::Result;
use lettre::{Message, SmtpTransport, Transport};
use crate::settings::{parse_creds, Settings};
use crate::otp::generate_otp;
use crate::envelope::Envelope;

pub fn prepare_message(envelope: Envelope) -> Message {

    println!("{:?}", envelope);

    // TODO: read to, from, subject and body from toml file
    let from = envelope.from;
    let to = envelope.to;

    let subject = envelope.subject;
    let otp = generate_otp();
    let body = format!("{}\n{}", otp, envelope.body);

    println!("otp: {} to: {}", otp, to);

    Message::builder()
        .from(from.parse().unwrap())
        .reply_to(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap()
}

pub fn send(message: Message) -> Result<()> {

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
    match mailer.send(&message) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }

    Ok(())
}