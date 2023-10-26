#![feature(slice_take)]
// #![no_main]

pub mod abi;
pub mod bytes;
pub mod evm;
pub mod fuzzing;
pub mod inspector;

// has linking problem otherwise
#[allow(unused)]
fn main() {}
