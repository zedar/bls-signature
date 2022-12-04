//! Secret and public key generator

use ark_bls12_381::Fr as ScalarField;
use ark_bls12_381::G2Affine;
use ark_ec::AffineCurve;
use ark_serialize::CanonicalSerialize;
use ark_std::UniformRand;

/// Keys structure represents secret and public key
pub struct Keys {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

/// Generate secret and public key using the BLS12-381 curve
/// https://hackmd.io/@benjaminion/bls12-381#BLS-digital-signatures
pub fn generate() -> Result<Keys, Box<dyn std::error::Error>> {
    let mut rng = ark_std::rand::thread_rng();

    // create secret key as a random number frm the scalar field (Fr)
    let secret_key = ScalarField::rand(&mut rng);

    let mut sk_bs = Vec::new();
    secret_key.serialize(&mut sk_bs).unwrap();
    println!("Secret key: {:?}", hex::encode_upper(&sk_bs));

    let public_key = G2Affine::prime_subgroup_generator().mul(secret_key);

    let mut pk_bs = Vec::new();
    public_key.serialize(&mut pk_bs).unwrap();
    println!("Public key: {:?}", hex::encode_upper(&pk_bs));

    Ok(Keys {
        secret_key: sk_bs,
        public_key: pk_bs,
    })
}
