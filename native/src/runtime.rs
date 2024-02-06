pub mod asset_holder;

use crate::{data_entry::DataEntry, vm::Vm};
use wasmi::Memory;

/// Structure allowing to accumulate arguments for calling a contract function.
pub struct Args {
    bytes: Vec<u8>,
    length: u16,
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

impl Args {
    /// Initialization of an empty set of function arguments.
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            length: 0,
        }
    }

    /// Getting byte representation of function arguments.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.length.to_be_bytes().to_vec();
        bytes.extend(self.bytes.clone());
        bytes
    }

    /// Adding an argument to call the contract function.
    pub fn push(&mut self, value: DataEntry) {
        self.bytes.extend(value.serialize(None));
        self.length += 1;
    }
}

/// Structure allowing to accumulate payments for calling a contract function.
pub struct Payments {
    bytes: Vec<u8>,
    length: u16,
}

impl Default for Payments {
    fn default() -> Self {
        Self::new()
    }
}

impl Payments {
    /// Initialization of an empty set of function payments.
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            length: 0,
        }
    }

    /// Getting byte representation of function payments.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.length.to_be_bytes().to_vec();
        bytes.extend(self.bytes.clone());
        bytes
    }

    /// Adding an payment to call the contract function.
    pub fn push(&mut self, asset_id: &[u8], amount: i64) {
        if asset_id.is_empty() {
            self.bytes.push(0);
        } else {
            self.bytes.push(1);
            self.bytes.extend_from_slice(asset_id);
        }

        self.bytes.extend_from_slice(&amount.to_be_bytes());
        self.length += 1;
    }
}

/// A structure accessible within the WASM interpreter to access linear memory,
/// call stack, heap address, and other necessary information.
pub struct Runtime<'a> {
    memory: Option<Memory>,
    pub vm: &'a mut Vm,
    heap_base: i32,
    pub args: Args,
    pub payments: Payments,
}

impl<'a> Runtime<'a> {
    pub fn new(vm: &'a mut Vm) -> Self {
        Self {
            memory: None,
            vm,
            heap_base: 0,
            args: Args::new(),
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

    /// Obtain a byte representation of the arguments and payments
    /// of the called function and zeroize the value for subsequent calls.
    pub fn args_and_payments(&mut self) -> (Vec<u8>, Vec<u8>) {
        let result = (self.args.as_bytes(), self.payments.as_bytes());
        self.args = Args::new();
        self.payments = Payments::new();
        result
    }
}
