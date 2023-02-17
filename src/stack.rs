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

    pub fn call(
        &mut self,
        bytecode: Vec<u8>,
        func_name: &str,
        func_args: &[String],
        envs: Vec<&impl Environment>,
    ) -> Result<Vec<Value>> {
        let frame = Frame { bytecode };

        self.push_frame(frame);

        self.run(func_name, func_args, envs)
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

        exec.execute(&func_name, func_args, envs, self)
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    fn top_frame(&self) -> &Frame {
        self.frames.last().unwrap_or(&self.first_frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use convert_case::{Case, Casing};
    use std::str;
    use wasmi::Caller;

    use crate::env_runtime;
    use crate::tests::wat2wasm;

    const MEMORY: (u32, u32) = (1, 1);

    env_runtime! {
        pub fn Test() {
            |mut _caller: Caller<Runtime>| {
                assert_eq!(2 + 2, 4);
            }
        }
    }

    #[test]
    fn test_simple_module() {
        let wat = r#"
            (module
                (func $getValue (result i32)
                    i32.const 42)
                (func (export "run") (result i32)
                    call $getValue
                    i32.const 1
                    i32.add))
        "#;

        let bytecode = wat2wasm(wat).expect("Error parse wat");
        let env = Test;

        let mut stack = Stack::new(bytecode, MEMORY).expect("Error create stack");
        let result = stack.run("run", &[], vec![&env]).expect("Error execution");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Value::I32(43));
    }

    env_runtime! {
        pub fn TestSetValue(value: u32) {
            |mut _caller: Caller<Runtime>| {
                assert_eq!(13, value);
            }
        }
    }

    #[test]
    fn test_import_set_value_module() {
        let wat = r#"
            (module
                (import "env" "test_set_value" (func $setValue (param i32)))
                (func (export "run")
                    i32.const 13
                    call $setValue))
        "#;

        let bytecode = wat2wasm(wat).expect("Error parse wat");
        let env = TestSetValue;

        let mut stack = Stack::new(bytecode, MEMORY).expect("Error create stack");
        let result = stack.run("run", &[], vec![&env]).expect("Error execution");

        assert_eq!(result.len(), 0);
    }

    env_runtime! {
        pub fn TestGetValue() -> u32 {
            |mut _caller: Caller<Runtime>| {
                13
            }
        }
    }

    #[test]
    fn test_import_get_value_module() {
        let wat = r#"
            (module
                (import "env" "test_get_value" (func $getValue (result i32)))
                (func (export "run") (result i32)
                    call $getValue
                    i32.const 1
                    i32.add))
        "#;

        let bytecode = wat2wasm(wat).expect("Error parse wat");
        let env = TestGetValue;

        let mut stack = Stack::new(bytecode, MEMORY).expect("Error create stack");
        let result = stack.run("run", &[], vec![&env]).expect("Error execution");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Value::I32(14));
    }

    env_runtime! {
        pub fn TestMemory(offset: u32, length: u32) {
            |mut caller: Caller<Runtime>| {
                let (memory, _ctx) = caller
                    .data()
                    .memory()
                    .expect("Error get memory")
                    .data_and_store_mut(&mut caller);

                let result = str::from_utf8(&memory[offset as usize..offset as usize + length as usize])
                    .expect("Error converts a slice of bytes to a string slice");

                assert_eq!("Hi", result);
            }
        }
    }

    #[test]
    fn test_memory_module() {
        let wat = r#"
            (module
                (import "env" "test_memory" (func $print (param i32 i32)))
                (import "env" "memory" (memory 1 1))
                (data (i32.const 0) "Hi")
                (func (export "run")
                    i32.const 0  ;; pass offset 0 to print
                    i32.const 2  ;; pass length 2 to print
                    call $print))
        "#;

        let bytecode = wat2wasm(wat).expect("Error parse wat");
        let env = TestMemory;

        let mut stack = Stack::new(bytecode, MEMORY).expect("Error create stack");
        let result = stack.run("run", &[], vec![&env]).expect("Error execution");

        assert_eq!(result.len(), 0);
    }

    env_runtime! {
        pub fn Call() -> i32 {
            |mut caller: Caller<Runtime>| {
                caller.data_mut().call_contract()
            }
        }
    }

    #[test]
    fn test_call_contract() {
        let wat = r#"
            (module
                (import "env" "call" (func $call (result i32)))
                (func (export "run") (result i32)
                    call $call
                    i32.const 1
                    i32.add))
        "#;

        let bytecode = wat2wasm(wat).expect("Error parse wat");
        let env = Call;

        let mut stack = Stack::new(bytecode, MEMORY).expect("Error create stack");
        let result = stack.run("run", &[], vec![&env]).expect("Error execution");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Value::I32(5));
    }
}
