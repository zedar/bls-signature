use bls_signature::signature;
use clap::Parser;
use log::{debug, error, info};

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let cfg = signature::ValidateConfig::parse();
    debug!("CFG: {:?}", cfg);

    let sig = match std::fs::read_to_string(cfg.sig_file) {
        Err(err) => {
            error!("Error reading signature file: {}", err);
            std::process::exit(1)
        }
        Ok(sig) => sig,
    };

    let pk = match std::fs::read_to_string(cfg.pk_file) {
        Err(err) => {
            error!("Error reading public key file: {}", err);
            std::process::exit(1)
        }
        Ok(pk) => pk,
    };

    let msg = match cfg.msg {
        Some(msg) => msg,
        None => match cfg.msg_file {
            None => {
                error!("Missing message or message file");
                std::process::exit(1)
            }
            Some(msg_file) => match std::fs::read_to_string(msg_file) {
                Err(err) => {
                    error!("Error reading message file: {}", err);
                    std::process::exit(1)
                }
                Ok(msg) => msg,
            },
        },
    };

    match signature::validate(pk, msg, sig) {
        Err(err) => {
            error!("Error validating signature: {}", err);
            std::process::exit(1)
        }
        Ok(true) => info!("VALID signature"),
        Ok(false) => info!("INVALID signature"),
    }
}
