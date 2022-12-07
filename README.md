BLS12-381 digital signature
===========================

Base on the [https://hackmd.io/@benjaminion/bls12-381#BLS-digital-signatures](https://hackmd.io/@benjaminion/bls12-381#BLS-digital-signatures).

[Rust](rust-lang.org)
[arkworks](https://github.com/orgs/arkworks-rs/repositories)


# generate_keys

Generates secret and public key. Secret key is a random scalar value from the range [0..p-1], where p is a prime number from the prime order group.
Public key is a point in G2 group calculates as secret key * G2 generator point.

# sign_message

For an input message has value (256 bit long) is calculated, using blake2s algorithm. Hash value is projected on the G1 group using Pedersen transformation.
Then signature is a point in G1 group calculated as secret key * hash on curve.

# validate_signature

Using the pairing, it calculates Signature (G1 point) * G2 generator point and compares it to message hash (G1 point) * public key (G2 point).
