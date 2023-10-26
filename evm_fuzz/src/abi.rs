use crate::{
    bytes::take_last,
    evm::CallRes,
    fuzzing::{Artifact, CallSeq, RetSeq},
};
use bytes::Bytes;
use ethabi::{
    ethereum_types::{H160, U256},
    Constructor, Function, ParamType, Token,
};
use std::{
    collections::{HashMap, HashSet},
    env,
    ffi::OsStr,
    fs::File,
    io::Read,
    path::PathBuf,
};
use walkdir::WalkDir;

/// print the result of a fuzzing campaign
pub fn print_res(res: RetSeq) {
    let status: Vec<_> = res
        .iter()
        .map(|r| {
            let cres = &r.1 .0; // safe coz both are equal anyway
            match cres {
                Some(cres) => match cres {
                    CallRes::Success(_, _, _) => "S",
                    CallRes::Revert(_) => "R",
                    CallRes::Halt(_) => "H",
                },
                None => "E",
            }
        })
        .collect();

    let hmap = res.iter().fold(HashMap::new(), |mut hmap, (seq, (r, _))| {
        if matches!(r, Some(CallRes::Revert(_)) | Some(CallRes::Halt(_))) {
            let entry: &mut (HashSet<&CallSeq>, HashSet<&CallRes>, usize) =
                hmap.entry(seq.func.signature()).or_default();
            entry.0.insert(seq);
            entry.1.insert(r.as_ref().unwrap());
            entry.2 += 1;
        }

        hmap
    });
    let tot: usize = hmap.values().map(|v| v.2).sum();
    hmap.iter().for_each(|(sig, creses)| {
        let times = creses.2;
        if times > (tot * 8 / 10) {
            // > 80% of occurences
            // println!("{:?}", creses);
            // println!("[WARN] {} has reverted {}x", sig, times);
        }
    });

    let mut count = 1usize;
    let mut status_iter = status.iter().enumerate();
    while let Some((i, stat)) = status_iter.next() {
        if let Some(nstat) = status.get(i + 1) {
            if stat == nstat {
                count += 1
            } else {
                // print_count_res(stat, count);
                count = 1;
            }
        } else {
            // is last
            // print_count_res(stat, count);
            assert!(status_iter.next().is_none());
        }
    }
}

#[allow(unused)]
fn print_count_res(status: &str, count: usize) {
    assert_ne!(count, 0, "skill issue");
    println!("{count}x {status} ");
}

/// Returns all the abi instance recursively at this path
pub fn abis(path: PathBuf) -> Vec<(String, Artifact)> {
    WalkDir::new(path)
        .into_iter()
        .map(|entry| entry.unwrap().into_path())
        .filter(|path| path.extension() == Some(OsStr::new("json")))
        .map(|path| {
            let mut file = File::open(path.clone()).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            (
                path.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .strip_suffix(".json")
                    .unwrap()
                    .to_owned(),
                serde_json::from_str::<Artifact>(&content).unwrap(),
            )
        })
        .collect()
}

pub fn encode_func_args(function: &Function, seed: &mut Vec<u8>) -> Vec<u8> {
    let inputs = &function
        .inputs
        .iter()
        .map(|input| to_token(&input.kind, seed))
        .collect::<Vec<_>>();

    function.encode_input(&inputs[..]).unwrap()
}

pub fn encode_constructor_args(
    bytecode: Bytes,
    constructor: &Constructor,
    data: &mut Vec<u8>,
) -> Bytes {
    let inputs = &constructor
        .inputs
        .iter()
        .map(|input| to_token(&input.kind, data))
        .collect::<Vec<_>>();

    constructor
        .encode_input(bytecode.into(), &inputs[..])
        .unwrap()
        .into()
}

pub fn encode_constructor_and_get(
    bytecode: Bytes,
    constructor: &Constructor,
    data: &mut Vec<u8>,
) -> (Bytes, Vec<Token>) {
    let inputs = constructor
        .inputs
        .iter()
        .map(|input| to_token(&input.kind, data))
        .collect::<Vec<_>>();

    (
        constructor
            .encode_input(bytecode.into(), &inputs[..])
            .unwrap()
            .into(),
        inputs,
    )
}

/// Generate a token from a param_type and arbitrary data
pub fn to_token(param_type: &ParamType, data: &mut Vec<u8>) -> Token {
    match (param_type, type_size(param_type)) {
        (param_type, Some(size)) => {
            let bytes_size = size / 8;
            let mut _data = take_last(data, bytes_size);
            assert_eq!(_data.len(), bytes_size); // sanity check
            match param_type {
                ParamType::Address => Token::Address(H160::from_slice(&_data)),
                ParamType::Int(_) => Token::Int(U256::from_big_endian(&_data)),
                ParamType::Uint(_) => Token::Uint(U256::from_big_endian(&_data)),
                ParamType::Bool => {
                    let last_byte = _data.first().unwrap_or(&0);
                    Token::Bool(last_byte % 2 == 1)
                }
                ParamType::FixedArray(param_type, elements) => {
                    let mut tokens = Vec::new();
                    for _ in 0..*elements {
                        let token = to_token(param_type, &mut _data);
                        tokens.push(token);
                    }

                    Token::FixedArray(tokens)
                }
                ParamType::FixedBytes(_bytes) => Token::FixedBytes(_data),
                ParamType::Tuple(inner_types) => {
                    let mut tokens = Vec::new();
                    for inner_type in inner_types {
                        let token = to_token(inner_type, &mut _data);
                        tokens.push(token);
                    }

                    Token::Tuple(tokens)
                }
                _ => unreachable!(),
            }
        }
        (param_type, None) => {
            let len = (take_last(data, 1).pop().unwrap() % 3) as usize * 32; // max array size of 2
            let mut _data = take_last(data, len);
            match param_type {
                ParamType::Bytes => Token::Bytes(_data),
                ParamType::String => {
                    let mut as_string: String = String::from_utf8_lossy(&_data).into();
                    if as_string.len() >= 100 {
                        while as_string.len() > 100 {
                            as_string.pop();
                        }
                    }

                    Token::String(as_string)
                }
                ParamType::Array(param_type) => {
                    let mut tokens = Vec::new();
                    while !_data.is_empty() {
                        let token = to_token(param_type, &mut _data);
                        tokens.push(token);
                    }
                    Token::Array(tokens)
                }
                _ => unreachable!(),
            }
        }
    }
}

pub fn truncate_bytes(data: &[u8], bytes: usize) -> (Vec<u8>, Vec<u8>) {
    if bytes > data.len() {
        let zeros = vec![0u8; bytes - data.len()];
        ([data, &zeros].concat(), Vec::new())
    } else {
        let (left, right) = data.split_at(bytes);
        (left.to_vec(), right.to_vec())
    }
}

/// Returns the size of static types as bits
pub fn type_size(param_type: &ParamType) -> Option<usize> {
    match param_type {
        ParamType::Address => Some(160),
        ParamType::Int(bits) | ParamType::Uint(bits) => Some(*bits),
        ParamType::FixedBytes(bytes) => Some(8 * bytes),
        ParamType::Bool => Some(1),
        ParamType::FixedArray(param, _size) => type_size(param).map(|size| 32 * size),
        ParamType::Tuple(params) => Some(
            params
                .iter()
                .map(|param| type_size(param).unwrap_or(0))
                .sum(),
        ),
        ParamType::Bytes | ParamType::String | ParamType::Array(_) => None,
    }
}

pub fn equivalent_abis(_where: &str) -> Vec<(Artifact, Artifact)> {
    let builds = abis(env::current_dir().unwrap().join(_where));
    let mut artifacts: Vec<(Artifact, Artifact)> = Vec::new();

    // fock it :(
    for (file1, build1) in builds.clone() {
        if let Some(id1) = file1.strip_prefix("sol") {
            'inner: for (file2, build2) in builds.clone() {
                if let Some(id2) = file2.strip_prefix("vy") {
                    if id1 == id2 {
                        artifacts.push((build1.clone(), build2));
                        break 'inner;
                    }
                };
            }
        };
    }

    artifacts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fuzzing::fuzz_abi;
    use ethabi::{Param, ParamType, StateMutability};

    #[test]
    fn single_fuzz() {
        let artifacts = equivalent_abis("../.build/");
        for (_i, abi) in artifacts.iter().enumerate() {
            println!("{:?} ...........", abi.0.source_id);
            let mut data = vec![0u8; 1000];
            fuzz_abi(abi, &mut data);
            println!("[OK]");
        }
    }

    #[test]
    fn func_encode_sig() {
        #[allow(deprecated)]
        let func = Function {
            name: String::from("oh_man"),
            inputs: vec![],
            outputs: vec![],
            constant: None,
            state_mutability: StateMutability::Payable,
        };
        let mut seed = vec![0; 0];
        encode_func_args(&func, &mut seed);
        dbg!(&seed);
        assert!(seed.is_empty());
    }

    #[test]
    fn func_encode_arg() {
        #[allow(deprecated)]
        let func = Function {
            name: String::from("oh_man"),
            inputs: vec![Param {
                name: String::from(""),
                kind: ParamType::Uint(256),
                internal_type: None,
            }],
            outputs: vec![],
            constant: None,
            state_mutability: StateMutability::Payable,
        };
        let mut seed = vec![0; 32];
        encode_func_args(&func, &mut seed);
        assert!(seed.is_empty());

        let mut seed = vec![0; 33];
        encode_func_args(&func, &mut seed);
        assert_eq!(seed.len(), 1);
    }
}
