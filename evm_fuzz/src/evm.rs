use crate::{abi, fuzzing::CallSeq, inspector::StackInspector};
use bytes::Bytes;
use ethabi::Constructor;
use hex_literal::hex;
use revm::{
    self,
    db::EmptyDB,
    primitives::{
        AccountInfo, Address, Env, Eval, ExecutionResult, Halt, Log, Output, ShanghaiSpec, SpecId,
        TransactTo, U256,
    },
    InMemoryDB, EVM,
};
use std::{fmt::Debug, hash::Hash};

#[derive(PartialEq, Eq, Clone)]
pub enum CallRes {
    Success(Bytes, Vec<Log>, Eval),
    Revert(Bytes),
    Halt(Halt),
}

impl Hash for CallRes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            CallRes::Success(b, l, e) => {
                b.hash(state);
                l.iter().for_each(|lo| {
                    lo.topics.hash(state);
                    lo.data.hash(state);
                    lo.address.hash(state);
                });
                match e {
                    Eval::Stop => "STOP".hash(state),
                    Eval::Return => "RET".hash(state),
                    Eval::SelfDestruct => "SD".hash(state),
                };
            }
            CallRes::Revert(b) => {
                b.hash(state);
            }
            CallRes::Halt(_h) => {
                "halt".hash(state); // TODO yes
            }
        }
    }
}

impl Debug for CallRes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallRes::Success(data, logs, eval) => {
                write!(
                    f,
                    "S(out: 0x{:02x}, logs: {:?}, eval: {:?})",
                    data, logs, eval
                )
            }
            CallRes::Revert(data) => write!(f, "R(out: 0x{:02x})", data),
            CallRes::Halt(reason) => write!(f, "H(reason: {:?})", reason),
        }
    }
}

pub struct Provider {
    evm: EVM<InMemoryDB>,
}

impl Default for Provider {
    fn default() -> Self {
        Self::new()
    }
}

impl Provider {
    pub fn new() -> Self {
        let mut evm = EVM::new();
        evm.db = Some(InMemoryDB::new(EmptyDB::new()));
        let info = AccountInfo {
            balance: U256::MAX,
            nonce: 0,
            ..Default::default()
        };
        evm.db()
            .unwrap()
            .insert_account_info(Self::deployer(), info);
        Self { evm }
    }

    pub fn deployer() -> Address {
        Address(hex!("6B175474E89094C44Da98b954EedeAC495271d0F").into())
    }

    fn env(&mut self) -> &mut Env {
        let env = &mut self.evm.env;
        env.cfg.spec_id = SpecId::SHANGHAI;
        env.tx.gas_priority_fee = None;
        env
    }

    pub fn get_deploy_code(
        &mut self,
        _bytecode: Bytes,
        constructor: Option<&Constructor>,
        data: &mut Vec<u8>,
    ) -> Bytes {
        if let Some(constructor) = constructor {
            let blen = _bytecode.len();
            let bytecode_ = abi::encode_constructor_args(_bytecode, constructor, data);
            assert!(bytecode_.len() >= blen);
            bytecode_
        } else {
            _bytecode
        }
    }

    pub fn deploy_contract(
        &mut self,
        source: &str,
        _bytecode: Bytes,
        constructor: Option<&Constructor>,
        data: &mut Vec<u8>,
    ) -> Option<Address> {
        let (bytecode, tokens) = if let Some(constructor) = constructor {
            let blen = _bytecode.len();
            let (bytecode_, tokens) = abi::encode_constructor_and_get(_bytecode, constructor, data);
            assert!(bytecode_.len() >= blen);
            (bytecode_, tokens)
        } else {
            (_bytecode, Vec::new())
        };

        let env = self.env();
        env.tx.caller = Self::deployer();
        env.tx.transact_to = TransactTo::create();
        let bytes = bytecode.to_vec().into();
        env.tx.data = bytes;
        env.validate_tx::<ShanghaiSpec>().unwrap();
        env.validate_block_env::<ShanghaiSpec>().unwrap();

        match self.evm.inspect_commit(StackInspector) {
            Ok(exec) => match exec {
                ExecutionResult::Success { output, .. } => match output {
                    Output::Create(_, addr) => {
                        if addr == Some(Address::ZERO) {
                            println!("[WARN] deploy failed");

                            None
                        } else {
                            let addr = addr.unwrap();
                            let info = self.evm.db().unwrap().load_account(addr).unwrap();
                            // assert!(info.exists());
                            // assert!(!info.is_empty());
                            info.info.balance = U256::MAX.div_ceil(U256::from(2));

                            Some(addr)
                        }
                    }
                    _ => unreachable!(),
                },
                ExecutionResult::Revert { output, .. } => {
                    // warn!("deploy revert {:?}", output);
                    println!("[WARN] deploy revert {:?} {}", output, source);
                    println!("[DEBUG] {:?}", tokens);
                    None
                }
                ExecutionResult::Halt { reason, .. } => {
                    // warn!("deploy halt {:?}", reason);
                    println!("[WARN] deploy halt {:?} {}", reason, source);
                    None
                }
            },
            Err(_) => {
                println!("[WARN] transact error {}", source);
                None
            }
        }
    }

    pub fn call(&mut self, addr: Address, seq: CallSeq) -> Option<CallRes> {
        let env = self.env();
        env.tx.transact_to = TransactTo::Call(addr);
        env.tx.data = seq.data.clone().into();
        env.tx.value = seq.value;
        env.tx.caller = seq.caller;
        env.tx.gas_price = U256::ZERO;
        env.block.timestamp = seq.timestamp;
        self.evm
            .db()
            .unwrap()
            .load_account(seq.caller)
            .unwrap()
            .info
            .balance = U256::MAX;
        match self.evm.inspect_commit(StackInspector) {
            Ok(res) => {
                let res = match res {
                    ExecutionResult::Success {
                        output,
                        logs,
                        reason,
                        ..
                    } => CallRes::Success(output.into_data().into(), logs, reason),
                    ExecutionResult::Revert { output, .. } => {
                        // println!("[INFO] call revert! data {:#?}", output.0);
                        CallRes::Revert(output.into())
                    }
                    ExecutionResult::Halt { reason, .. } => {
                        println!("[WARN] call halt! {:?}", reason);
                        CallRes::Halt(reason)
                    }
                };

                Some(res)
            }
            Err(err) => {
                // println!(
                //     "[WARN] err {} !\n{:#?}\nbalance: {:?}",
                //     err,
                //     seq,
                //     self.evm
                //         .db()
                //         .unwrap()
                //         .load_account(seq.caller)
                //         .unwrap()
                //         .info
                //         .balance
                // );
                println!("[WARN] err {} !", err);
                None
            }
        }
    }
}
