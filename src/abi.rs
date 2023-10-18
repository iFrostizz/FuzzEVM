use ethabi::ethereum_types::{H160, U256};
use ethabi::{Contract, FixedBytes, Function, ParamType, Token};
use ethers_solc::artifacts::CompactContractBytecode;
use std::{ffi::OsStr, fs::File, io::Read, path::PathBuf};
use walkdir::WalkDir;

/// Returns all the abi instance recursively at this path
pub fn abis(path: PathBuf) -> Vec<Contract> {
    WalkDir::new(path)
        .into_iter()
        .map(|entry| entry.unwrap().into_path())
        .filter(|path| path.extension() == Some(OsStr::new("json")))
        .map(|path| {
            let mut file = File::open(path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            serde_json::from_str::<CompactContractBytecode>(&content)
                .unwrap()
                .abi
                .unwrap()
        })
        .collect()
}

pub fn encode_func_args(function: &Function, seed: &[u8]) -> Vec<u8> {
    let input_kinds = &function
        .inputs
        .iter()
        .map(|input| input.kind.clone())
        .collect();
    let unpacked = unpack_args(input_kinds);
    let inputs = unpacked
        .iter()
        .map(|param_type| {
            let _seed = seed; // TODO keccak or some shiet
            to_token(param_type, _seed)
        })
        .collect::<Vec<_>>();

    function.encode_input(&inputs[..]).unwrap()
}

pub fn unpack_args(params: &Vec<ParamType>) -> Vec<&ParamType> {
    params
        .iter()
        .flat_map(|param| {
            if let ParamType::Tuple(inner_params) = param {
                unpack_args(inner_params)
            } else {
                vec![param]
            }
        })
        .collect()
}

/// Generate a token from a param_type and arbitrary data
pub fn to_token(param_type: &ParamType, data: &[u8]) -> Token {
    match (param_type, type_size(param_type)) {
        (param_type, Some(size)) => match param_type {
            ParamType::Address => Token::Address(H160::from_slice(truncate_bytes(data, size))),
            // TODO arbitrary length bytes array
            ParamType::Int(_) => Token::Int(U256::from_big_endian(truncate_bytes(data, size))),
            ParamType::Uint(_) => Token::Uint(U256::from_big_endian(truncate_bytes(data, size))),
            ParamType::Bool => {
                let last_byte = data.get(0).unwrap_or(&0);
                Token::Bool(last_byte % 2 == 1)
            }
            ParamType::FixedBytes(_) => Token::FixedBytes(truncate_bytes(data, size).to_vec()),
            ParamType::FixedArray(param_tyme, elements) => Token::FixedArray(
                (0..(*elements))
                    .into_iter()
                    .map(|_| to_token(param_type, data))
                    .collect(),
            ),
            _ => unreachable!(),
        },
        (param_type, _) => match param_type {
            ParamType::Bytes => Token::Bytes(data.into()),
            ParamType::String => {
                let as_string = String::from_utf8_lossy(data.into()).into();
                Token::String(as_string)
            }
            ParamType::Array(types) => {
                // let length =
            }
            _ => unreachable!(),
        },
    }
}

pub fn truncate_bytes(data: &[u8], bits: usize) -> &[u8] {
    &data[0..(bits / 8)]
}

pub fn type_size(param_type: &ParamType) -> Option<usize> {
    match param_type {
        ParamType::Address => Some(160),
        ParamType::Bytes | ParamType::String | ParamType::Array(_) => None,
        ParamType::Int(bits) | ParamType::Uint(bits) | ParamType::FixedBytes(bits) => Some(*bits),
        ParamType::Bool => Some(1),
        ParamType::FixedArray(_, size) => Some(32 + 32 * size),
        ParamType::Tuple(params) => Some(
            params
                .iter()
                .map(|param| type_size(param).unwrap_or(0))
                .sum(),
        ),
    }
}
