// generate a 6 digit random number between 100,000 and 999,999 to use as a one-time-passkey
pub fn generate_otp() -> u64 {
    let range = 100_000..1_000_000_u64;
    fastrand::u64(range)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_otp() {
        for _i in 0..10 {
            let otp = generate_otp();
            println!("{}", otp);
        }
    }
}
