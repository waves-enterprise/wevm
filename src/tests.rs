use crate::{
    env_runtime,
    jvm::Jvm,
    runtime::{Environment, Runtime},
    stack::Stack,
};
use convert_case::{Case, Casing};
use std::str;
use wasmi::{Caller, Func, Store};

// Test implementation of JVM calls from the stack
impl Jvm for Stack {
    fn jvm_get_bytecode(&self, _name: &str) -> Vec<u8> {
        let wat = r#"
            (module
                (func (export "run") (result i32)
                    i32.const 2
                    i32.const 2
                    i32.add))
            "#;

        wat2wasm(wat).expect("WAT code parsing failed")
    }
}

env_runtime! {
    pub fn Test() {
        |mut _caller: Caller<Runtime>| {
            assert_eq!(2 + 2, 4);
        }
    }
}

env_runtime! {
    pub fn TestSetValue(value: u32) {
        |mut _caller: Caller<Runtime>| {
            assert_eq!(42, value);
        }
    }
}

env_runtime! {
    pub fn TestGetValue() -> u32 {
        |mut _caller: Caller<Runtime>| {
            42
        }
    }
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

/// Converts the given `.wat` into `.wasm`.
pub fn wat2wasm(wat: &str) -> Result<Vec<u8>, wat::Error> {
    wat::parse_str(wat)
}
