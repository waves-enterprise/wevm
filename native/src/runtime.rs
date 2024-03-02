pub mod asset_holder;
pub mod data_entry;
pub mod params;
pub mod payment_id;
pub mod payments;
pub mod utils;

use crate::vm::Vm;
use params::Params;
use payments::Payments;
use wasmi::Memory;

/// A structure accessible within the WASM interpreter to access linear memory,
/// call stack, heap address, and other necessary information.
pub struct Runtime<'a> {
    memory: Option<Memory>,
    pub vm: &'a mut Vm,
    heap_base: i32,
    pub params: Params,
    pub payments: Payments,
}

impl<'a> Runtime<'a> {
    pub fn new(vm: &'a mut Vm) -> Self {
        Self {
            memory: None,
            vm,
            heap_base: 0,
            params: Params::new(),
            payments: Payments::new(),
        }
    }

    /// Getting linear memory.
    pub fn memory(&self) -> Option<Memory> {
        self.memory
    }

    /// Setting linear memory.
    pub fn set_memory(&mut self, memory: Memory) {
        self.memory = Some(memory);
    }

    /// Getting the heap address from a contract.
    pub fn heap_base(&self) -> i32 {
        self.heap_base
    }

    /// Setting the value of the heap address of the contract.
    pub fn set_heap_base(&mut self, value: i32) {
        self.heap_base = value;
    }
}
