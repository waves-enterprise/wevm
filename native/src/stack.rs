use crate::{
    env::Environment,
    exec::{Executable, LoadableFunction},
    Result,
};
use jni::{objects::GlobalRef, JavaVM};
use std::str::FromStr;
use wasmi::core::Value;

pub struct Frame {
    bytecode: Vec<u8>,
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
        bytecode: Vec<u8>,
        memory: (u32, u32),
        envs: Vec<Box<dyn Environment>>,
        jvm: JavaVM,
        jvm_callback: GlobalRef,
    ) -> Result<Self> {
        let first_frame = Frame { bytecode };

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
        bytecode: Vec<u8>,
        func_name: &str,
        input_data: Vec<u8>,
    ) -> Result<Vec<Value>> {
        let frame = Frame { bytecode };

        self.push_frame(frame);
        self.run(func_name, input_data)
    }

    pub fn run(&mut self, func_name: &str, input_data: Vec<u8>) -> Result<Vec<Value>> {
        let frame = self.top_frame();

        let func_name = LoadableFunction::from_str(func_name)?;

        let exec = Executable::new(frame.bytecode.clone(), self.memory.0, self.memory.1)?;
        exec.execute(&func_name, input_data, self.envs.clone(), self)
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    fn top_frame(&self) -> &Frame {
        self.frames.last().unwrap_or(&self.first_frame)
    }
}
