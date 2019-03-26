use bip39::{Mnemonic, Language};

use sodiumoxide::crypto::hash::sha256;
use sodiumoxide::crypto::sign::ed25519;
use sodiumoxide::crypto::sign::ed25519::Seed;

#[macro_use]
extern crate clap;

fn kp(phrase: &str) {
    // recover seed
    let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
    let seed_slice = mnemonic.entropy();
    println!("Seed: {:?}", hex::encode(seed_slice));
    if seed_slice.len() != 32 {
        panic!("unexpected seed of length {} (expected: 32)", seed_slice.len());
    }

    // generate ED25519 KeyPair
    let seed = Seed::from_slice(seed_slice).unwrap();
    let (public, secret) = ed25519::keypair_from_seed(&seed);

    // print secret- and public key
    println!("Secret Key: {}", hex::encode(&secret[..]));
    println!("Public Key: {}", hex::encode(&public));
}

fn hash(input: &str) {
    let digest = sha256::hash(input.as_bytes());
    println!("SHA256 Hash: {:?}", hex::encode(digest));
}

fn main() {
    let matches = clap_app!(hesod =>
        (@subcommand kp =>
            (@arg MNEMONIC: +required "Sets the mnemonic to generate a ED25519 key pair from")
        )
        (@subcommand hash =>
            (@arg INPUT: +required "input to hash")
        )
    ).get_matches();
    match matches.subcommand_name() {
        Some("kp") => kp(matches.subcommand_matches("kp").unwrap().value_of("MNEMONIC").unwrap()),
        Some("hash") => hash(matches.subcommand_matches("hash").unwrap().value_of("INPUT").unwrap()),
        _ => panic!("unknown command triggered"),
    };
}
