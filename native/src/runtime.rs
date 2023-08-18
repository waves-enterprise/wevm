use crate::stack::Stack;
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

pub struct Runtime<'a> {
    memory: Option<Memory>,
    pub stack: &'a mut Stack,
    heap_base: i32,
}

impl<'a> Runtime<'a> {
    pub fn new(stack: &'a mut Stack) -> Self {
        Runtime {
            memory: None,
            stack,
            heap_base: 0,
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
}
