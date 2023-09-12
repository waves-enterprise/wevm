use crate::{
    env::Environment,
    exec::{Executable, LoadableFunction},
    Result,
};
use jni::{objects::GlobalRef, JavaVM};
use std::str::FromStr;
use wasmi::core::Value;

pub struct Frame {
    contract_id: Vec<u8>,
    bytecode: Vec<u8>,
}

impl Frame {
    pub fn contract_id(&self) -> Vec<u8> {
        self.contract_id.clone()
    }
}

// TODO: It is necessary to limit the number of possible frames
pub struct Stack {
    frames: Vec<Frame>,
    first_frame: Frame,
    memory: (u32, u32),
    envs: Vec<Box<dyn Environment>>,
    pub jvm: JavaVM,
    pub jvm_callback: GlobalRef,
}

impl Stack {
    pub fn new(
        contract_id: Vec<u8>,
        bytecode: Vec<u8>,
        memory: (u32, u32),
        envs: Vec<Box<dyn Environment>>,
        jvm: JavaVM,
        jvm_callback: GlobalRef,
    ) -> Result<Self> {
        let first_frame = Frame {
            contract_id,
            bytecode,
        };

        Ok(Stack {
            frames: Default::default(),
            first_frame,
            memory,
            envs,
            jvm,
            jvm_callback,
        })
    }

    pub fn call(
        &mut self,
        contract_id: Vec<u8>,
        bytecode: Vec<u8>,
        func_name: &str,
        input_data: Vec<u8>,
    ) -> Result<Vec<Value>> {
        let frame = Frame {
            contract_id,
            bytecode,
        };

        self.push_frame(frame);
        self.run(func_name, input_data)
    }

    pub fn run(&mut self, func_name: &str, input_data: Vec<u8>) -> Result<Vec<Value>> {
        let frame = self.top_frame();

        let func_name = LoadableFunction::from_str(func_name)?;

        let exec = Executable::new(frame.bytecode.clone(), self.memory.0, self.memory.1)?;
        let result = exec.execute(&func_name, input_data, self.envs.clone(), self);

        self.frames.pop();

        result
    }

    pub fn top_frame(&self) -> &Frame {
        self.frames.last().unwrap_or(&self.first_frame)
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }
}
