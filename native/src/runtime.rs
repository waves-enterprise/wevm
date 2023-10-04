use crate::{data_entry::DataEntry, stack::Stack};
use wasmi::Memory;

#[derive(Copy, Clone, Debug)]
pub enum RuntimeError {
    MemoryNotFound = 300,
    MemoryError = 301,
    Utf8Error = 302,
    InvalidResult = 303,
    InvalidInteger = 304,
    InvalidBool = 305,
    Base58Error = 306,
}

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
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            length: 0,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.length.to_be_bytes().to_vec();
        bytes.extend(self.bytes.clone());
        bytes
    }

    pub fn push(&mut self, value: DataEntry) {
        self.bytes.extend(value.serialize(None));
        self.length += 1;
    }
}

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
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            length: 0,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.length.to_be_bytes().to_vec();
        bytes.extend(self.bytes.clone());
        bytes
    }

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

pub struct Runtime<'a> {
    memory: Option<Memory>,
    pub stack: &'a mut Stack,
    heap_base: i32,
    pub args: Args,
    pub payments: Payments,
}

impl<'a> Runtime<'a> {
    pub fn new(stack: &'a mut Stack) -> Self {
        Self {
            memory: None,
            stack,
            heap_base: 0,
            args: Args::new(),
            payments: Payments::new(),
        }
    }

    pub fn memory(&self) -> Option<Memory> {
        self.memory
    }

    pub fn set_memory(&mut self, memory: Memory) {
        self.memory = Some(memory);
    }

    pub fn heap_base(&self) -> i32 {
        self.heap_base
    }

    pub fn set_heap_base(&mut self, value: i32) {
        self.heap_base = value;
    }

    pub fn args_and_payments(&mut self) -> (Vec<u8>, Vec<u8>) {
        let result = (self.args.as_bytes(), self.payments.as_bytes());
        self.args = Args::new();
        self.payments = Payments::new();
        result
    }
}
