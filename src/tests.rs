use wasmi::Memory;

use crate::runtime::Runtime;

/// Converts the given `.wat` into `.wasm`.
pub fn wat2wasm(wat: &str) -> Result<Vec<u8>, wat::Error> {
    wat::parse_str(wat)
}

/// Runtime implementation for testing.
#[derive(Clone, Copy)]
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
}
