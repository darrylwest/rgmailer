

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

pub fn send(settings: Settings, message: Message) -> Result<()> {

    let creds = parse_creds(settings.clone());

    // Open a remote connection to gmail
    let host = settings.clone().smtp.host;
    let mailer = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&message) {
        Ok(_) => return Ok(()),
        Err(e) => {
            eprint!("Could not send email: {:?}", e);
            return Err(e.into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prep_message() {
        let filename = "tests/test-message.toml";
        let envelope = Envelope::read_file(filename).unwrap();

        println!("env: {:?}", envelope);

        let message = prepare_message(envelope);

        println!("msg: {:?}", message);
    }

    #[test]
    fn sent_it() {
        let settings = Settings::read(Some(String::from("tests/test-settings.toml"))).unwrap();
        let filename = "tests/test-message.toml";
        let envelope = Envelope::read_file(filename).unwrap();
        let message = prepare_message(envelope);

        let resp = send(settings, message);
        println!("resp: {:?}", resp);
        assert!(true);
    }
}
