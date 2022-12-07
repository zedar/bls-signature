use ark_serialize::Write;
use bls_signature::signature;
use clap::Parser;

fn main() {
    let cfg = signature::Config::parse();
    println!("CFG: {:?}", cfg);

    let sk = match std::fs::read_to_string(cfg.sk_file.as_path()) {
        Err(err) => {
            println!("Error reading secret key file: {}", err);
            std::process::exit(1)
        }
        Ok(sk) => sk,
    };

    let msg = match cfg.msg {
        Some(msg) => msg,
        None => match cfg.msg_file {
            None => {
                println!("Missing message or message file");
                std::process::exit(1)
            }
            Some(msg_file) => match std::fs::read_to_string(msg_file.as_path()) {
                Err(err) => {
                    println!("Error reading message file: {}", err);
                    std::process::exit(1)
                }
                Ok(msg) => msg,
            },
        },
    };

    let sig = match signature::sign(sk, msg) {
        Err(err) => {
            println!("Error signing a message: {}", err);
            std::process::exit(1)
        }
        Ok(sig) => sig,
    };

    if cfg.sig_file.as_path().exists() {
        if !cfg.overwrite {
            println!(
                "Signature file already exists: {}",
                cfg.sig_file.as_path().display()
            );
            std::process::exit(1)
        } else {
            if let Err(err) = std::fs::remove_file(cfg.sig_file.as_path()) {
                println!("Error removing signature file: {}", err);
                std::process::exit(1)
            }
        }
    }

    let mut sig_file = match std::fs::File::create(cfg.sig_file.as_path()) {
        Err(err) => {
            println!("Error creating signature file: {}", err);
            std::process::exit(1)
        }
        Ok(file) => file,
    };

    match sig_file.write_all(hex::encode_upper(&sig).as_bytes()) {
        Err(err) => {
            println!("Error writing signature file: {}", err);
            std::process::exit(1)
        }
        Ok(_) => println!("Signature file created"),
    }
}
