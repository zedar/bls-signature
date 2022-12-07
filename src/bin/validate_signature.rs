use bls_signature::signature;
use clap::Parser;

fn main() {
    let cfg = signature::ValidateConfig::parse();
    println!("CFG: {:?}", cfg);

    let sig = match std::fs::read_to_string(cfg.sig_file) {
        Err(err) => {
            println!("Error reading signature file: {}", err);
            std::process::exit(1)
        }
        Ok(sig) => sig,
    };

    let pk = match std::fs::read_to_string(cfg.pk_file) {
        Err(err) => {
            println!("Error reading public key file: {}", err);
            std::process::exit(1)
        }
        Ok(pk) => pk,
    };

    let msg = match cfg.msg {
        Some(msg) => msg,
        None => match cfg.msg_file {
            None => {
                println!("Missing message or message file");
                std::process::exit(1)
            }
            Some(msg_file) => match std::fs::read_to_string(msg_file) {
                Err(err) => {
                    println!("Error reading message file: {}", err);
                    std::process::exit(1)
                }
                Ok(msg) => msg,
            },
        },
    };

    match signature::validate(pk, msg, sig) {
        Err(err) => {
            println!("Error validating signature: {}", err);
            std::process::exit(1)
        }
        Ok(true) => println!("VALID signature"),
        Ok(false) => println!("INVALID signature"),
    }
}
