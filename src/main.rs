use hex_literal::hex;
use sha2::{Digest, Sha256};
use std::{
    env,
    fmt::{Debug, LowerHex},
};

fn main() {
    let args: Vec<_> = env::args().collect();

    match args.len() {
        2 => {
            let mut bytes = [0u8; 64];
            hex::decode_to_slice(&args[1], &mut bytes as &mut [u8])
                .expect("please provide a valid 64-byte hex string");
            if let Ok((prefix, solution)) = simple_pow(bytes) {
                println!("{:x}\n{:08x}", solution, prefix);
            }
        }
        _ => help(),
    }
}

fn help() {
    println!("provide a 64 byte hex as the sole parameter");
}

fn simple_pow(bytes: [u8; 64]) -> Result<(u32, impl Debug + LowerHex), ()> {
    const CAFE: [u8; 2] = hex!("cafe");

    for prefix in 0..u32::MAX {
        let mut hasher = Sha256::new();
        hasher.update(prefix.to_be_bytes());
        hasher.update(bytes);
        let finalized = hasher.finalize();
        if CAFE == finalized[finalized.len() - 2..finalized.len()] {
            return Ok((prefix, finalized));
        }
    }
    Err(())
}

#[test]
fn test_my_power() {
    const INPUT: [u8; 64] = hex!("129df964b701d0b8e72fe7224cc71643cf8e000d122e72f742747708f5e3bb6294c619604e52dcd8f5446da7e9ff7459d1d3cefbcc231dd4c02730a22af9880c");
    const EXPECTED_PREFIX: [u8; 4] = hex!("00003997");
    const EXPECTED_SOLUTION: &str =
        "6681edd1d36af256c615bf6dcfcda03c282c3e0871bd75564458d77c529dcafe";
    let (prefix, solution) = simple_pow(INPUT).expect("we should find a solution at some point");
    assert_eq!(prefix.to_be_bytes(), EXPECTED_PREFIX);
    assert_eq!(format!("{:x}", solution), EXPECTED_SOLUTION);
}
