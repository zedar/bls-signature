use ark_serialize::Write;
use bls_signature::signature;
use clap::Parser;
use log::{debug, error, info};

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let cfg = signature::Config::parse();
    debug!("CFG: {:?}", cfg);

    let sk = match std::fs::read_to_string(cfg.sk_file.as_path()) {
        Err(err) => {
            error!("Error reading secret key file: {}", err);
            std::process::exit(1)
        }
        Ok(sk) => sk,
    };

    let msg = match cfg.msg {
        Some(msg) => msg,
        None => match cfg.msg_file {
            None => {
                error!("Missing message or message file");
                std::process::exit(1)
            }
            Some(msg_file) => match std::fs::read_to_string(msg_file.as_path()) {
                Err(err) => {
                    error!("Error reading message file: {}", err);
                    std::process::exit(1)
                }
                Ok(msg) => msg,
            },
        },
    };

    let sig = match signature::sign(sk, msg) {
        Err(err) => {
            error!("Error signing a message: {}", err);
            std::process::exit(1)
        }
        Ok(sig) => sig,
    };

    if cfg.sig_file.as_path().exists() {
        if !cfg.overwrite {
            error!(
                "Signature file already exists: {}",
                cfg.sig_file.as_path().display()
            );
            std::process::exit(1)
        } else {
            if let Err(err) = std::fs::remove_file(cfg.sig_file.as_path()) {
                error!("Error removing signature file: {}", err);
                std::process::exit(1)
            }
        }
    }

    let mut sig_file = match std::fs::File::create(cfg.sig_file.as_path()) {
        Err(err) => {
            error!("Error creating signature file: {}", err);
            std::process::exit(1)
        }
        Ok(file) => file,
    };

    match sig_file.write_all(hex::encode_upper(&sig).as_bytes()) {
        Err(err) => {
            error!("Error writing signature file: {}", err);
            std::process::exit(1)
        }
        Ok(_) => info!("Signature file created"),
    }
}
