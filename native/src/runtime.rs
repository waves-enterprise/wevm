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
}

impl<'a> Runtime<'a> {
    pub fn new(stack: &'a mut Stack) -> Self {
        Runtime {
            memory: None,
            stack,
        }
    }

    pub fn memory(&self) -> Option<Memory> {
        self.memory
    }

    pub fn set_memory(&mut self, memory: Memory) {
        self.memory = Some(memory);
    }
}
