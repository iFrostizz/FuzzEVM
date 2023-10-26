#![no_main]
#![feature(lazy_cell)]

use evm_fuzz::{
    abi::{self},
    bytes::take_last,
    fuzzing::{self, Artifact},
};
use libfuzzer_sys::fuzz_target;
use std::sync::{LazyLock, Mutex};

static ABIS: LazyLock<Mutex<Vec<(Artifact, Artifact)>>> = LazyLock::new(|| {
    let artifacts = abi::equivalent_abis(".build/");
    println!("fuzzing {} equivalences", artifacts.len());
    Mutex::new(artifacts)
});

fuzz_target!(|data: &[u8]| {
    let mut data = data.to_vec();
    let abis = ABIS.lock().unwrap();
    let idx = take_last(&mut data, 1)[0] as usize % abis.len();
    fuzzing::fuzz_abi(&abis[idx], &mut data);
});
