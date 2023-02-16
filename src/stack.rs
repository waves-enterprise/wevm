use std::str::FromStr;
use wasmi::{core::Value, Caller, Func, Store};

use crate::{
    exec::{Executable, LoadableFunction},
    runtime::{Environment, Runtime},
    Result,
};

pub struct Frame {
    bytecode: Vec<u8>,
}

pub struct Stack {
    frames: Vec<Frame>,
    first_frame: Frame,
    memory: (u32, u32),
}

impl Stack {
    pub fn new(bytecode: Vec<u8>, memory: (u32, u32)) -> Result<Self> {
        let first_frame = Frame { bytecode };

        Ok(Stack {
            frames: Default::default(),
            first_frame,
            memory,
        })
    }

    pub fn run(
        &mut self,
        func_name: &str,
        func_args: &[String],
        envs: Vec<&impl Environment>,
    ) -> Result<Vec<Value>> {
        let frame = self.top_frame();

        let func_name = LoadableFunction::from_str(func_name)?;

        let exec = Executable::new(frame.bytecode.clone(), self.memory.0, self.memory.1)?;

        exec.execute(&func_name, func_args, envs, Some(self))
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    fn top_frame(&self) -> &Frame {
        self.frames.last().unwrap_or(&self.first_frame)
    }
}

impl Environment for Stack {
    fn module(&self) -> String {
        String::from("env")
    }

    fn name(&self) -> String {
        String::from("call")
    }

    // TODO: To implement the creation of a new frame and run
    fn func(&self, store: &mut Store<Runtime>) -> Func {
        Func::wrap(store, |mut caller: Caller<Runtime>| -> i32 { 43 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use convert_case::{Case, Casing};
    use wasmi::Caller;

    use crate::env_runtime;
    use crate::tests::wat2wasm;

    env_runtime! {
        pub fn Test() {
            |mut _caller: Caller<Runtime>| {
                assert_eq!(2 + 2, 4);
            }
        }
    }

    #[test]
    fn test_stack() {
        let wat = r#"
            (module
                (import "env" "call" (func $call (result i32)))
                (func (export "run") (result i32)
                    call $call
                    i32.const 1
                    i32.add))
        "#;

        let bytecode = wat2wasm(wat).expect("Error parse wat");

        let func_name = "run";
        let func_args: [String; 0] = [];

        let memory = (1, 1);
        let env = Test;

        let mut stack = Stack::new(bytecode, memory).expect("Error create stack");

        let result = stack
            .run(func_name, &func_args, vec![&env])
            .expect("Error execution");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Value::I32(44));
    }
}
