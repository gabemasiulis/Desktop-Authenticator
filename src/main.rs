use std::io;
use totp_rs::{Algorithm, TOTP, Secret};

fn main() {

    let mut secret_input = String::new();
    match io::stdin().read_line(&mut secret_input) {
        Ok(n) => {
            println!("Secret of {n} bytes input:");
            println!("{secret_input}");
            println!();
        }
        Err(error) => {
            println!("Error: {error}");
            println!();
        }
    }
    // stdin adds a line feed that we need to remove before creating a secret
    secret_input.pop();

    let secret = Secret::Encoded(secret_input);


    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret.to_bytes().unwrap()
    ).unwrap();
    let token = totp.generate_current().unwrap();
    println!("{}", token);

}
