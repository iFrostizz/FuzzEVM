#[allow(unused)]
use revm::{
    interpreter::{opcode, OpCode},
    primitives::{ruint::Uint, U256},
    Database, Inspector,
};

#[derive(Debug, Default)]
pub struct StackInspector;

impl<DB: Database> Inspector<DB> for StackInspector {
    fn step(
        &mut self,
        _interp: &mut revm::interpreter::Interpreter,
        _data: &mut revm::EVMData<'_, DB>,
    ) -> revm::interpreter::InstructionResult {
        // if let Some(op) = OpCode::new(_interp.current_opcode()) {
        //     // dbg!(&op.as_str());

        //     let mut stack = _interp.stack().data().clone();
        //     stack.reverse();

        //     let cdc = OpCode::new(opcode::CALLDATACOPY).unwrap();
        //     let cc = OpCode::new(opcode::CODECOPY).unwrap();

        //     if op == cdc {
        //         check_warning(stack.get(1), "offset");
        //         check_warning(stack.get(2), "size");
        //     } else if op == cc {
        //         check_warning(stack.get(0), "destOffset");
        //         check_warning(stack.get(1), "offset");
        //         check_warning(stack.get(2), "size");
        //     }
        // };

        revm::interpreter::InstructionResult::Continue
    }
}

#[allow(unused)]
fn check_warning(el: Option<&Uint<256, 4>>, name: &str) {
    if let Some(val) = el {
        if *val >= U256::from(u32::MAX) {
            println!("[WARN] {} is high {:?}", name, val);
        }
    }
}
