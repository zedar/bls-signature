use std::{io::Cursor, ops::Neg};

use ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
use ark_ec::{AffineCurve, PairingEngine};
use ark_ff::One;
use ark_serialize::CanonicalDeserialize;

use crate::hash::hash_to_curve;

pub fn validate(pk: String, msg: String, sig: String) -> Result<bool, Box<dyn std::error::Error>> {
    let pk_bytes = hex::decode(pk)?;
    let pk_gr = G2Affine::deserialize(&mut Cursor::new(pk_bytes))?;

    let sig_bytes = hex::decode(sig)?;
    let sig_gr = G1Affine::deserialize(&mut Cursor::new(sig_bytes))?;

    // calculate message hash projected to the G1 affine group
    let msg_curve_hash = hash_to_curve(msg.as_bytes())?;

    // calculate product of pairings

    Ok(Bls12_381::product_of_pairings(&[
        (
            sig_gr.into(),
            G2Affine::prime_subgroup_generator().neg().into(),
        ),
        (msg_curve_hash.into(), pk_gr.into()),
    ])
    .is_one())
}
