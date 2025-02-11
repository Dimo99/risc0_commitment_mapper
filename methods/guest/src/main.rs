#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};

risc0_zkvm::guest::entry!(main);

fn bench_sha_n(n_thousands: u64) {
    let arr = [123u8; 1000];
    for _ in 0..n_thousands {
        let mut hasher = Sha256::new();
        hasher.update(arr);
        let _ = hasher.finalize();
    }
}

pub fn main() {
    let n = env::read();
    bench_sha_n(n);
}
