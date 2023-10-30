use lettre::transport::smtp::authentication::Credentials;
use std::env;

// TODO: change to RGMAILER with host, username, pw and return as a tuple
const AUTH_KEY: &str = "EMAIL_CREDS";

pub fn read_creds() -> Credentials {
    // read from key store
    use base64::decode;

    let b64 = env::var(AUTH_KEY).expect("should read creds from env");
    let plain = decode(b64).expect("should decode the b64");
    let plain = String::from_utf8(plain).expect("should be string");

    // println!("{}", &plain);
    let v: Vec<&str> = plain.split(':').collect();

    let username = v[0].to_string();
    let password = v[1].to_string();

    // TODO: refactor to just return generic username and password
    Credentials::new(username, password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_creds() {
        let creds = read_creds();
        println!("creds: {:?}", creds)
    }
}
