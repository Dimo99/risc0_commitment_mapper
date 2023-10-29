#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let index: u64 = env::read();
    let index_count: u64 = env::read();
    let seed: [u8; 32] = env::read();

    let shuffled_index = compute_shuffled_index(index, index_count, &seed);
    env::commit(&shuffled_index);
}

const SHUFFLE_ROUND_COUNT: u8 = 90;

pub fn compute_shuffled_index(index: u64, index_count: u64, seed: &[u8]) -> u64 {
    assert!(index < index_count);

    let mut index = index;
    for round in 0..SHUFFLE_ROUND_COUNT {
        let pivot = bytes_to_int64(&hash_with_round(seed, round)[..]) % index_count;
        index = do_round(seed, index, pivot, round, index_count);
    }

    index
}

fn do_round(seed: &[u8], index: u64, pivot: u64, round: u8, index_count: u64) -> u64 {
    let flip = (pivot + (index_count - index)) % index_count;
    let position = max(index, flip);
    let source = hash_with_round_and_position(seed, round, position);
    let byte = source[((position % 256) / 8) as usize];
    let bit = (byte >> (position % 8)) % 2;
    if bit == 1 {
        flip
    } else {
        index
    }
}

fn bytes_to_int64(slice: &[u8]) -> u64 {
    let mut bytes = [0; 8];
    bytes.copy_from_slice(&slice[0..8]);
    u64::from_le_bytes(bytes)
}

fn hash_with_round_and_position(seed: &[u8], round: u8, position: u64) -> [u8; 32] {
    let mut hasher = Sha256::new();

    hasher.update(seed);
    hasher.update(&[round]);
    // Code and comment was copied from lighthouse IDK if it should be checked
    /*
     * Note: the specification has an implicit assertion in `int_to_bytes4` that `position / 256 <
     * 2**24`. For efficiency, we do not check for that here as it is checked in `compute_shuffled_index`.
     */
    hasher.update(&(position / 256).to_le_bytes()[0..4]);

    let digest = hasher.finalize();
    digest.into()
}

fn hash_with_round(seed: &[u8], round: u8) -> [u8; 32] {
    let mut hasher = Sha256::new();

    hasher.update(seed);
    hasher.update(&[round]);

    let digest = hasher.finalize();
    digest.into()
}

fn max(a: u64, b: u64) -> u64 {
    if a > b {
        a
    } else {
        b
    }
}
