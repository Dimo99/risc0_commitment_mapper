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

use host::compute_shuffled_index;
use methods::COMPUTE_SHUFFLED_INDEX_ID;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let index: u64 = args[1].parse().unwrap();
    let index_count: u64 = args[2].parse().unwrap();

    let (receipt, _) = compute_shuffled_index(
        index,
        index_count,
        &[
            93, 207, 54, 169, 97, 51, 202, 45, 162, 64, 37, 169, 246, 134, 39, 32, 230, 5, 180, 18,
            110, 55, 228, 91, 21, 136, 204, 158, 16, 172, 175, 108,
        ],
    );

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify(COMPUTE_SHUFFLED_INDEX_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
