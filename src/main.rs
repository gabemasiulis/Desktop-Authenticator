use std::io;
use totp_rs::{Algorithm, TOTP, Secret};

fn main() {

    // TODO consider stack instead of heap? secret gets heaped upon encoding.
    // accept input of hmac secret
    let mut secret_input = String::new();
    match io::stdin().read_line(&mut secret_input) {
        Ok(n) => {
            println!("Secret of {n} bytes input:");
            println!("{secret_input}");
        }
        Err(error) => {
            println!("Error: {error}");
            println!();
        }
    }

    // sanitize hmac secret
    secret_input = secret_input.to_uppercase().chars().filter(|c| c.is_alphanumeric()).collect();

    // encode as secret
    let secret = Secret::Encoded(secret_input);


    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret.to_bytes().unwrap()
    ).unwrap();

    // generate the current OTP
    let token = totp.generate_current().unwrap();
    println!("Current OTP: {}", token);

}
