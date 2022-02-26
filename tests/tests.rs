// Copyright 2019-2021 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

use sha3::{Digest, Keccak256};

#[evm_precompiles_derive::generate_function_selector]
pub enum Action {
    Toto = "toto()",
    Tata = "tata()",
}

#[test]
fn tests() {
    assert_eq!(
        &(Action::Toto as u32).to_be_bytes()[..],
        &Keccak256::digest(b"toto()")[0..4],
    );
    assert_eq!(
        &(Action::Tata as u32).to_be_bytes()[..],
        &Keccak256::digest(b"tata()")[0..4],
    );
    assert_ne!(Action::Toto as u32, Action::Tata as u32);
}
