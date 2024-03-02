use crate::{
    error::{Error, ExecutableError, Result},
    exec::{Executable, LoadableFunction},
    modules::Module,
    runtime::payment_id::PaymentId,
};
use jni::{objects::GlobalRef, JavaVM};
use std::str::FromStr;
use wasmi::Value;

const MAX_FRAMES: usize = 64;

/// A frame of the call stack that stores the `contract_id` and `bytecode` of the contract.
pub struct Frame {
    contract_id: Vec<u8>,
    bytecode: Vec<u8>,
    nonce: u64,
}

impl Frame {
    pub fn contract_id(&self) -> Vec<u8> {
        self.contract_id.clone()
    }

    pub fn payment_id(&self) -> Vec<u8> {
        PaymentId::new(self.contract_id.clone(), self.nonce).as_bytes()
    }
}

/// The entry point for the virtual machine.
/// Contains:
/// * Call stack
/// * WASM environment
/// * Interface to interact with node (or simulation)
pub struct Vm {
    frames: Vec<Frame>,
    first_frame: Frame,
    memory: (u32, u32),
    modules: Vec<Module>,
    pub jvm: Option<JavaVM>,
    pub jvm_callback: Option<GlobalRef>,
    nonce: u64,
}

impl Vm {
    /// VM initialization.
    /// During initialization, the first contract is placed on the stack of the call.
    pub fn new(
        contract_id: Vec<u8>,
        bytecode: Vec<u8>,
        memory: (u32, u32),
        modules: Vec<Module>,
        jvm: Option<JavaVM>,
        jvm_callback: Option<GlobalRef>,
    ) -> Result<Self> {
        let first_frame = Frame {
            contract_id,
            bytecode,
            nonce: 0,
        };

        Ok(Self {
            frames: Default::default(),
            first_frame,
            memory,
            modules,
            jvm,
            jvm_callback,
            nonce: 0,
        })
    }

    /// Calling another contract when a contract is executed.
    /// Contract is placed on top of the call stack.
    pub fn call(
        &mut self,
        contract_id: Vec<u8>,
        bytecode: Vec<u8>,
        nonce: u64,
        func_name: &str,
        params: &[u8],
    ) -> Result<Vec<Value>> {
        let frame = Frame {
            contract_id,
            bytecode,
            nonce,
        };

        self.push_frame(frame)?;
        self.run(func_name, params)
    }

    /// Run contract. The contract is taken from the top of the call stack.
    pub fn run(&mut self, func_name: &str, params: &[u8]) -> Result<Vec<Value>> {
        let frame = self.top_frame();

        let func_name = LoadableFunction::from_str(func_name)?;

        let exec = Executable::new(&frame.bytecode, self.memory.0, self.memory.1)?;
        let result = exec.execute(&func_name, params, self.modules.clone(), self);

        self.frames.pop();

        result
    }

    /// Getting the frame at the top of the call stack.
    pub fn top_frame(&self) -> &Frame {
        self.frames.last().unwrap_or(&self.first_frame)
    }

    pub fn get_nonce(&mut self) -> u64 {
        self.nonce += 1;
        self.nonce
    }

    /// Get the caller of the current frame.
    pub fn get_caller_current_frame(&self) -> Vec<u8> {
        if self.frames.is_empty() {
            vec![]
        } else {
            let len = self.frames.len();

            match self.frames.get(len - 2) {
                Some(frame) => frame.contract_id(),
                None => self.first_frame.contract_id(),
            }
        }
    }

    fn push_frame(&mut self, frame: Frame) -> Result<()> {
        if self.frames.len() == MAX_FRAMES {
            return Err(Error::Executable(ExecutableError::StackOverflow));
        }

        self.frames.push(frame);

        Ok(())
    }
}
