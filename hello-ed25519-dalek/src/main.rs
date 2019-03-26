use std::env;
use std::string::String;

use bip39::{Mnemonic, Language};

use rand::{SeedableRng, StdRng};
use ed25519_dalek::Keypair;

fn main() {
    // parse CLI arguments
    let args: Vec<String> = env::args().collect();
    println!("ARGS: {:?}", args);
    if args.len() != 2 {
        panic!("expected 1 arguments, not {}", args.len()-1);
    }

    // recover seed
    let phrase = &args[1];
    let mnemonic = Mnemonic::from_phrase(phrase.as_str(), Language::English).unwrap();
    let seed = mnemonic.entropy();
    println!("Seed: {:?}", hex::encode(seed));
    if seed.len() != 32 {
        panic!("unexpected seed of length {} (expected: 32)", seed.len());
    }

    // generate ED25519 KeyPair
    let mut array = [0; 32];
    array.copy_from_slice(seed); 
    let mut rng: StdRng = SeedableRng::from_seed(array);
    let kp = Keypair::generate(&mut rng);

    // print secret- and public key
    let secret = kp.secret.as_bytes();
    println!("Secret Key: {} (len: {})", hex::encode(secret), secret.len());
    let public = kp.public.as_bytes();
    println!("Public Key: {} (len: {})", hex::encode(public), public.len());
}
