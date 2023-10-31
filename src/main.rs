use anyhow::Result;
use rgmailer::mailer::prepare_message;
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

    return prepare_message();
}
