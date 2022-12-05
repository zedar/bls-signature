use ark_bls12_381::{G1Affine, G1Projective};
use ark_crypto_primitives::{
    crh::pedersen::{Window, CRH},
    CRH as CRHScheme,
};
use blake2s_simd::blake2s;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[derive(Clone)]
struct PedersenWindow {}

impl Window for PedersenWindow {
    const WINDOW_SIZE: usize = 1;
    const NUM_WINDOWS: usize = 256;
}

pub fn hash_to_curve(msg: &[u8]) -> Result<G1Affine, Box<dyn std::error::Error>> {
    let rng_pedersen = &mut ChaCha20Rng::from_seed([
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1,
    ]);

    let params = CRH::<G1Projective, PedersenWindow>::setup(rng_pedersen)?;
    let b2hash = blake2s(msg);

    CRH::<G1Projective, PedersenWindow>::evaluate(&params, b2hash.as_bytes())
}
