// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use methods::COMPUTE_SHUFFLED_INDEX_ELF;
use risc0_zkvm::{
    default_prover,
    serde::{from_slice, to_vec},
    ExecutorEnv, Receipt,
};

pub fn compute_shuffled_index(index: u64, index_count: u64, seed: &[u8; 32]) -> (Receipt, u64) {
    let env = ExecutorEnv::builder()
        .write(&index)
        .unwrap()
        .write(&index_count)
        .unwrap()
        .write(&seed)
        .unwrap()
        .build()
        .unwrap();

    // println!("po2 {:?}", env.segment_limit_po2);

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover
        .prove(env, COMPUTE_SHUFFLED_INDEX_ELF)
        .unwrap()
        .receipt;

    let r: u64 = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    // println!("SEED IS {:?}", seed);
    println!("The compute shuffled index is {}", r);

    (receipt, r)
}
