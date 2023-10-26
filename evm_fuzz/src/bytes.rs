use tiny_keccak::{Hasher, Keccak};

/// take the last x bytes of a bytemap
pub fn take_last(data: &mut Vec<u8>, depth: usize) -> Vec<u8> {
    (0..depth).map(|_| data.pop().unwrap_or(0)).collect()
}

/// make more data with less data
pub fn mutate_data(data: &[u8]) -> Vec<u8> {
    data.iter()
        .flat_map(|byte| {
            let mut keccak = Keccak::v256();
            keccak.update(&[*byte]);
            let mut output = [0u8; 32];
            keccak.finalize(&mut output);
            output.to_vec()
        })
        .collect()
}

pub fn shuffle_byte(byte: u8) -> u8 {
    mutate_data(&[byte])[0]
}
