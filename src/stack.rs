use crate::{
    exec::{Executable, LoadableFunction},
    runtime::Environment,
    Result,
};
use std::str::FromStr;
use wasmi::core::Value;

pub struct Frame {
    bytecode: Vec<u8>,
}

// TODO: It is necessary to limit the number of possible frames
pub struct Stack {
    frames: Vec<Frame>,
    first_frame: Frame,
    memory: (u32, u32),
    envs: Vec<Box<dyn Environment>>,
}

impl Stack {
    pub fn new(
        bytecode: Vec<u8>,
        memory: (u32, u32),
        envs: Vec<Box<dyn Environment>>,
    ) -> Result<Self> {
        let first_frame = Frame { bytecode };

        Ok(Stack {
            frames: Default::default(),
            first_frame,
            memory,
            envs,
        })
    }

    pub fn call(
        &mut self,
        bytecode: Vec<u8>,
        func_name: &str,
        func_args: &[String],
    ) -> Result<Vec<Value>> {
        let frame = Frame { bytecode };

        self.push_frame(frame);
        self.run(func_name, func_args)
    }

    pub fn run(&mut self, func_name: &str, func_args: &[String]) -> Result<Vec<Value>> {
        let frame = self.top_frame();

        let func_name = LoadableFunction::from_str(func_name)?;

        let exec = Executable::new(frame.bytecode.clone(), self.memory.0, self.memory.1)?;
        exec.execute(&func_name, func_args, self.envs.clone(), self)
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

    use crate::{
        env_runtime,
        runtime::{get_envs, Runtime},
        tests::wat2wasm,
    };
    use convert_case::{Case, Casing};
    use std::str;
    use wasmi::{Caller, Func, Store};

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
        let test = Test;

        let mut stack =
            Stack::new(bytecode, MEMORY, vec![Box::new(test)]).expect("Error create stack");
        let result = stack.run("run", &[]).expect("Error execution");

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
        let test_set_value = TestSetValue;

        let mut stack = Stack::new(bytecode, MEMORY, vec![Box::new(test_set_value)])
            .expect("Error create stack");
        let result = stack.run("run", &[]).expect("Error execution");

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
        let test_get_value = TestGetValue;

        let mut stack = Stack::new(bytecode, MEMORY, vec![Box::new(test_get_value)])
            .expect("Error create stack");
        let result = stack.run("run", &[]).expect("Error execution");

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
                    i32.const 0  ;; Pass offset 0 to test
                    i32.const 2  ;; Pass length 2 to test
                    call $print))
        "#;

        let bytecode = wat2wasm(wat).expect("Error parse wat");
        let test_memory = TestMemory;

        let mut stack =
            Stack::new(bytecode, MEMORY, vec![Box::new(test_memory)]).expect("Error create stack");
        let result = stack.run("run", &[]).expect("Error execution");

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_call_contract() {
        let wat = r#"
            (module
                (import "env" "call_contract" (func $call (param i32 i32 i32 i32 i32 i32) (result i32)))
                (import "env" "memory" (memory 1 1))
                (func (export "run") (result i32)
                    (call $call
                        (i32.const 0) ;; Offset address of the called contract
                        (i32.const 3) ;; Length of the called contract
                        (i32.const 3) ;; Offset address of the function name
                        (i32.const 3) ;; Length of the function name
                        (i32.const 6) ;; Offset address of the function args
                        (i32.const 4) ;; Length of the function args
                    ))

                ;; Called contract
                (data (i32.const 0) "two")

                ;; Function name
                (data (i32.const 3) "run")

                ;; Function args
                (data (i32.const 6) "\01\02\03\04")
            )
        "#;

        let bytecode = wat2wasm(wat).expect("Error parse wat");
        let envs = get_envs();

        let mut stack = Stack::new(bytecode, MEMORY, envs).expect("Error create stack");
        let result = stack.run("run", &[]).expect("Error execution");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Value::I32(1));
    }
}
