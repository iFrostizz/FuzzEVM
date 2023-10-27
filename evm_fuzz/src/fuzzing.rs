use crate::{
    abi::{encode_func_args, print_res},
    bytes::{mutate_data, shuffle_byte, take_last},
    evm::{CallRes, Provider},
};
use bytes::Bytes;
use ethabi::Function;
use ethers_solc::artifacts::LosslessAbi;
use revm::primitives::{Address, U256};
use serde::Deserialize;
use std::{fmt::Debug, hash::Hash};

#[derive(Clone, PartialEq)]
pub struct CallSeq {
    pub func: Function,
    pub data: Bytes,
    pub caller: Address,
    pub value: U256,
    pub timestamp: U256,
}

impl Eq for CallSeq {}

impl Hash for CallSeq {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.func.signature().hash(state);
        self.data.hash(state);
        self.caller.hash(state);
        self.value.hash(state);
        self.timestamp.hash(state);
    }
}

impl Debug for CallSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallSeq")
            .field("func", &self.func.signature())
            .field("data", &format!("0x{:02x}", self.data))
            .field("caller", &format!("{:02x?}", self.caller))
            .field("value", &self.value)
            .field("timestamp", &self.timestamp)
            .finish()
    }
}

pub type RetSeq = Vec<(CallSeq, (Option<CallRes>, Option<CallRes>))>;

#[derive(Debug, Deserialize, Clone)]
pub struct Bytecode {
    pub bytecode: String,
}

#[derive(Deserialize, Clone)]
pub struct Artifact {
    pub abi: Option<LosslessAbi>,
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,
    #[serde(rename = "deploymentBytecode")]
    pub bytecode: Option<Bytecode>,
}

/// run a fuzz for two instances to compare
pub fn fuzz_abi(artifacts: &(Artifact, Artifact), data: &mut Vec<u8>) {
    let res = exec_seq(artifacts, data);
    print_res(res);
}

pub fn exec_seq<'a>(artifacts: &'a (Artifact, Artifact), data: &'a mut Vec<u8>) -> RetSeq {
    let (art0, art1) = artifacts;
    let source0 = &art0.source_id.clone().unwrap();
    let source1 = &art1.source_id.clone().unwrap();
    let names = format!("{} {}", source0, source1);

    let (bytecode0, bytecode1) = (
        art0.clone().bytecode.unwrap(),
        art1.clone().bytecode.unwrap(),
    );

    let (abi0, abi1) = (art0.clone().abi.unwrap().abi, art1.clone().abi.unwrap().abi);
    let (constructor0, constructor1) = (abi0.constructor(), abi1.constructor());
    let (bytecode0, bytecode1) = (get_bytecode(bytecode0), get_bytecode(bytecode1));

    let functions0: Vec<_> = abi0.functions.values().flatten().cloned().collect();
    let functions1: Vec<_> = abi1.functions.values().flatten().cloned().collect();
    assert_eq!(functions0.len(), functions1.len(), "{}", names);

    let (mut prov0, mut prov1) = (Provider::default(), Provider::default());
    let mut seqs = Vec::new();

    // only exhaust data if it has constructor inputs
    let mut constr_data = if constructor0.is_some_and(|c| !c.inputs.is_empty())
        || constructor1.is_some_and(|c| !c.inputs.is_empty())
    {
        mutate_data(&take_last(data, 32))
    } else {
        Default::default()
    };

    if let (Some(addr0), Some(addr1)) = (
        prov0.deploy_contract(source0, bytecode0, constructor0, &mut constr_data.clone()),
        prov1.deploy_contract(source1, bytecode1, constructor1, &mut constr_data),
    ) {
        for seq in rand_seq(functions0, &mut data.clone()) {
            let out0 = prov0.call(addr0, seq.clone());
            let out1 = prov1.call(addr1, seq.clone());
            assert_eq!(out0, out1, "{} {:#?}", names, seq);
            seqs.push((seq, (out0, out1)));
        }

        for seq in rand_seq(functions1, data) {
            let out0 = prov0.call(addr0, seq.clone());
            let out1 = prov1.call(addr1, seq.clone());
            assert_eq!(out0, out1, "{} {:#?}", names, seq);
            seqs.push((seq, (out0, out1)));
        }
    } else {
        println!("[WARN] deployment failed!");
    }

    seqs
}

fn get_bytecode(bytecode: Bytecode) -> Bytes {
    hex::decode(bytecode.bytecode.strip_prefix("0x").unwrap())
        .unwrap()
        .into()
}

pub fn rand_seq(functions: Vec<Function>, data: &mut Vec<u8>) -> Vec<CallSeq> {
    let func_len = functions.len();
    assert!(func_len > 0);
    let mut func_id = u8::from_be_bytes(take_and_add(data, 1, 1).try_into().unwrap());
    let seq_len = if func_len > 5 {
        5_usize.pow(5)
    } else {
        func_len.pow(func_len as u32)
    };

    // let mut seqs = Vec::new();

    let func = &functions[func_id as usize % func_len];
    vec![CallSeq {
        func: func.clone(),
        data: encode_func_args(func, data).into(),
        caller: Address::from_slice(&take_last(data, 20)),
        value: U256::ZERO,
        timestamp: U256::from_be_bytes::<32>(take_and_add(data, 8, 32).try_into().unwrap()),
    }]

    // for i in 0..seq_len {
    //     if data.is_empty() {
    //         return seqs;
    //     } else {
    //         func_id = shuffle_byte(func_id);
    //         let func = functions
    //             .get((i * func_id as usize) % func_len)
    //             .unwrap()
    //             .clone();
    //         // let value = U256::from_be_bytes::<32>(take_and_add(data, 8, 32).try_into().unwrap());
    //         // let caller = Address::from_slice(&take_last(data, 20));
    //         // let timestamp =
    //         //     U256::from_be_bytes::<32>(take_and_add(data, 8, 32).try_into().unwrap());
    //         let value = U256::ZERO;
    //         let caller = Address::from_slice(&take_last(data, 20));
    //         let timestamp =
    //             U256::from_be_bytes::<32>(take_and_add(data, 8, 32).try_into().unwrap());

    //         let calldata: Bytes = encode_func_args(&func, data).into();

    //         seqs.push(CallSeq {
    //             func,
    //             data: calldata,
    //             caller,
    //             value,
    //             timestamp,
    //         });
    //     }
    // }

    // seqs
}

pub fn take_and_add(data: &mut Vec<u8>, take_size: usize, ret_size: usize) -> Vec<u8> {
    add_trailing_zeros(take_last(data, take_size), ret_size)
}

fn add_trailing_zeros(data: Vec<u8>, size: usize) -> Vec<u8> {
    if size < data.len() {
        panic!("panik!");
    } else {
        (data.len()..size).map(|_| 0).chain(data).collect()
    }
}

#[allow(unused)]
fn pick_and(data: &mut [u8], mask: &[u8]) -> Vec<u8> {
    mask.iter()
        .rev()
        .enumerate()
        .map(|(i, byte)| {
            let data_byte = if !data.is_empty() {
                if data.len() > i {
                    data[data.len() - 1 - i]
                } else {
                    0
                }
            } else {
                0
            };

            data_byte & byte
        })
        .rev()
        .collect()
}

#[cfg(test)]
mod test {
    use crate::bytes::mutate_data;

    #[test]
    fn mutate_constructor() {
        let data = mutate_data(&[0u8; 32]);
        assert_ne!(data, vec![0u8; 32]);
    }
}
