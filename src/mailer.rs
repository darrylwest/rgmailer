use crate::envelope::Envelope;
use crate::settings::{parse_creds, Settings};
use anyhow::Result;
use lettre::{Message, SmtpTransport, Transport};
use log::{error, info};

pub fn prepare_message(envelope: Envelope) -> Message {
    info!("to: {}, subjecy: {}", envelope.to, envelope.subject);

    // TODO: read to, from, subject and body from toml file
    let from = envelope.from;
    let to = envelope.to;

    let subject = envelope.subject;
    let body = envelope.body;

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
        Ok(_) => Ok(()),
        Err(e) => {
            let msg = format!("Could not send email: {}", e);
            eprint!("\nError! {}\n", msg);
            error!("{}", msg);
            Err(e.into())
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
