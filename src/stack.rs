use crate::{
    exec::{Executable, LoadableFunction},
    runtime::Environment,
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        runtime::CallContract,
        tests::{wat2wasm, Test, TestCallContract, TestGetValue, TestMemory, TestSetValue},
    };
    use jni::{InitArgsBuilder, JNIVersion, JavaVM};

    #[test]
    fn test_vm() {
        let wat = r#"
        (module
            (import "env0" "test_set_value" (func $test_set_value (param i32)))
            (import "env0" "test_get_value" (func $test_get_value (result i32)))
            (import "env0" "test_memory" (func $test_memory (param i32 i32)))
            (import "env0" "test_call_contract" (func $test_call_contract (param i32)))

            (import "env0" "call_contract" (func $call (param i32 i32 i32 i32 i32 i32) (result i32)))

            (import "env" "memory" (memory 1 1))

            (func (export "_constructor"))

            (func (export "run") (param $p0 i64) (result i32)
                (call $test_set_value
                    (call $test_get_value)
                    (i32.wrap_i64
                        (local.get $p0))
                    (i32.add))

                (call $test_memory
                    (i32.const 0)  ;; Pass offset 0 to test
                    (i32.const 2)) ;; Pass length 2 to test

                (call $test_call_contract
                    (call $call
                        (i32.const 2)   ;; Offset address of the called contract
                        (i32.const 3)   ;; Length of the called contract
                        (i32.const 5)   ;; Offset address of the function name
                        (i32.const 3)   ;; Length of the function name
                        (i32.const 8)   ;; Offset address of the function args
                        (i32.const 20))) ;; Length of the function args

                (i32.const 0)
            )

            ;; For test memory
            (data (i32.const 0) "Hi")

            ;; Called contract
            (data (i32.const 2) "two")

            ;; Function name
            (data (i32.const 5) "sum")

            ;; Function args
            (data (i32.const 8) "\01\00\00\00\00\00\00\00\00\00\00\02")
        )
        "#;

        let bytecode = wat2wasm(wat).expect("WAT code parsing failed");
        let memory: (u32, u32) = (1, 1);

        // Preparing a fake jvm to initialize the call stack
        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option("-Xcheck:jni")
            .build()
            .expect("Failed to initialize JVM args");
        let java_vm = JavaVM::new(jvm_args).expect("JavaVM initialization failed");

        let env = java_vm
            .attach_current_thread()
            .expect("Failed attaches the current thread to the Java VM");

        let jvm = env
            .get_java_vm()
            .expect("Failed receiving JavaVM interface");
        let array = env.new_byte_array(1).expect("Array creation failed");
        let global_ref = env
            .new_global_ref(array)
            .expect("Error callback new_global_ref");

        // Test imports
        let test = Test;
        let test_set_value = TestSetValue;
        let test_get_value = TestGetValue;
        let test_memory = TestMemory;
        let test_call_contract = TestCallContract;
        let call_contract = CallContract;

        let mut stack = Stack::new(
            bytecode,
            memory,
            vec![
                Box::new(test),
                Box::new(test_set_value),
                Box::new(test_get_value),
                Box::new(test_memory),
                Box::new(test_call_contract),
                Box::new(call_contract),
            ],
            jvm,
            global_ref,
        )
        .expect("Call stack creation failed");

        let input_data: Vec<u8> = vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ];

        let result = stack
            .run("run", input_data)
            .expect("Bytecode execution failed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Value::I32(0));
    }
}
