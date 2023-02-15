use wasmi::Memory;

use crate::runtime::Runtime;

/// Converts the given `.wat` into `.wasm`.
pub fn wat2wasm(wat: &str) -> Result<Vec<u8>, wat::Error> {
    wat::parse_str(wat)
}

/// Runtime implementation for testing.
pub struct MockRuntime {
    memory: Option<Memory>,
}

impl Runtime for MockRuntime {
    fn new() -> Self {
        MockRuntime { memory: None }
    }

    fn memory(&self) -> Option<Memory> {
        self.memory
    }

    fn set_memory(&mut self, memory: Memory) {
        self.memory = Some(memory);
    }

    fn get_contract(&self) -> Vec<u8> {
        get_bytecode()
    }
}

fn get_bytecode() -> Vec<u8> {
    let wat = r#"
        (module
            (func (export "calc") (result i32)
                i32.const 2
                i32.const 2
                i32.add))
    "#;

    let bytecode = wat2wasm(wat).expect("Error parse wat");

    bytecode
}
