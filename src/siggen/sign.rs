use std::io::Cursor;

use ark_bls12_381::Fr as ScalarField;
use ark_ec::AffineCurve;
use ark_serialize::CanonicalDeserialize;
use ark_serialize::CanonicalSerialize;

use crate::hash::hash_to_curve;

pub fn sign(sk: String, msg: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let sk_bytes = hex::decode(sk)?;
    let msg_bytes = msg.as_bytes();

    // calculate message hash projected to the G1 affine group
    let msg_curve_hash = hash_to_curve(msg_bytes)?;

    // calculate message signature: sk*msg_hash => G1Affine
    let sk_scalarf = ScalarField::deserialize_uncompressed(&mut Cursor::new(sk_bytes))?;
    let sig = msg_curve_hash.mul(sk_scalarf);

    let mut sig_bs = Vec::new();
    sig.serialize(&mut sig_bs)?;
    println!("Signature: {:?}", hex::encode_upper(&sig_bs));

    Ok(sig_bs)
}
