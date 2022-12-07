use ark_serialize::Write;
use bls_signature::keys;
use clap::Parser;
use log::{debug, error, info};

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let cfg = keys::Config::parse();
    debug!("CFG: {:?}", cfg);

    let keys = match keys::generate() {
        Err(err) => {
            error!("Error: {}", err);
            std::process::exit(1)
        }
        Ok(keys) => keys,
    };

    // store keys to the files
    let mut sk_path = cfg.target_path.clone();
    sk_path.push("sk.txt");
    if sk_path.as_path().exists() {
        if !cfg.overwrite {
            error!(
                "Secret key file already exists: {}",
                sk_path.as_path().display()
            );
            std::process::exit(1)
        } else {
            if let Err(err) = std::fs::remove_file(sk_path.as_path()) {
                error!("Error removing secret key file: {}", err);
                std::process::exit(1)
            }
        }
    }

    let mut pk_path = cfg.target_path.clone();
    pk_path.push("pk.txt");
    if pk_path.as_path().exists() {
        if !cfg.overwrite {
            error!(
                "Public key file already exists: {}",
                pk_path.as_path().display()
            );
            std::process::exit(1)
        } else {
            if let Err(err) = std::fs::remove_file(pk_path.as_path()) {
                error!("Error removing public key file: {}", err);
                std::process::exit(1)
            }
        }
    }

    let mut sk_file = match std::fs::File::create(sk_path.as_path()) {
        Err(err) => {
            error!("Error creating secret key file: {}", err);
            std::process::exit(1)
        }
        Ok(file) => file,
    };

    match sk_file.write_all(hex::encode_upper(&keys.secret_key).as_bytes()) {
        Err(err) => {
            error!("Error writing secret key file: {}", err);
            std::process::exit(1)
        }
        Ok(_) => info!("Secret key file created"),
    }

    let mut pk_file = match std::fs::File::create(pk_path.as_path()) {
        Err(err) => {
            error!("Error creating public key file: {}", err);
            std::process::exit(1)
        }
        Ok(file) => file,
    };

    match pk_file.write_all(hex::encode(&keys.public_key).as_bytes()) {
        Err(err) => {
            error!("Error writing public key file: {}", err);
            std::process::exit(1)
        }
        Ok(_) => info!("Public key file created!"),
    }
}
