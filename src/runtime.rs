use crate::{jvm::Jvm, stack::Stack};
use convert_case::{Case, Casing};
use dyn_clone::DynClone;
use jni::sys::jint;
use std::str;
use wasmi::{core::Value, Caller, Func, Memory, Store};

pub trait Environment: DynClone {
    fn module(&self) -> String;
    fn name(&self) -> String;
    fn func(&self, store: &mut Store<Runtime>) -> Func;
}

dyn_clone::clone_trait_object!(Environment);

pub struct Runtime<'a> {
    memory: Option<Memory>,
    stack: &'a mut Stack,
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

    pub fn get_bytecode(&mut self, name: &str) -> Vec<u8> {
        self.stack.jvm_get_bytecode(name)
    }

    pub fn call_contract(
        &mut self,
        bytecode: Vec<u8>,
        func_name: &str,
        func_args: &[String],
    ) -> i32 {
        let result = self
            .stack
            .call(bytecode, func_name, func_args)
            .expect("Bytecode execution failed");

        match result[0] {
            Value::I32(value) => value as jint,
            _ => 0 as jint,
        }
    }
}

#[macro_export]
macro_rules! env_runtime {
    ( pub fn $name:ident ( $($args:tt)* ) $(-> $return_values:ty)? { $func:expr } ) => {
        #[derive(Clone)]
        pub struct $name;

        impl Environment for $name {
            // TODO: We may have to use versioning for future updates
            fn module(&self) -> String {
                String::from("env")
            }

            fn name(&self) -> String {
                let name = stringify!($name);
                name.from_case(Case::Pascal).to_case(Case::Snake)
            }

            fn func(&self, store: &mut Store<Runtime>) -> Func {
                Func::wrap(
                    store,
                    |caller: Caller<Runtime>, $($args)*| $(-> $return_values)? {
                        $func(caller)
                    }
                )
            }
        }
    }
}

env_runtime! {
    pub fn CallContract(offset_contract: u32, length_contract: u32, offset_func_name: u32, length_func_name: u32, offset_func_args: u32, length_func_args: u32) -> i32 {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = caller
                    .data()
                    .memory()
                    .expect("Error get memory")
                    .data_and_store_mut(&mut caller);

            let contract_name = str::from_utf8(&memory[offset_contract as usize..offset_contract as usize + length_contract as usize])
                .expect("Error converts a slice of bytes to a string slice");
            let bytecode = ctx.get_bytecode(contract_name);

            let func_name = str::from_utf8(&memory[offset_func_name as usize..offset_func_name as usize + length_func_name as usize])
                .expect("Error converts a slice of bytes to a string slice");

            // TODO: Parse args
            let func_args: [String; 0] = [];

            ctx.call_contract(bytecode, func_name, &func_args)
        }
    }
}

pub fn get_envs() -> Vec<Box<dyn Environment>> {
    let mut envs: Vec<Box<dyn Environment>> = vec![];

    let call_contract = CallContract;
    envs.push(Box::new(call_contract));

    envs
}
