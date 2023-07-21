use crate::{jvm::Jvm, stack::Stack};
use convert_case::{Case, Casing};
use dyn_clone::DynClone;
use std::str;
use wasmi::{Caller, Func, Memory, Store};

#[derive(Copy, Clone, Debug)]
pub enum RuntimeError {
    MemoryNotFound = 300,
    Utf8Error = 301,
}

pub trait Environment: DynClone {
    fn module(&self) -> String;
    fn name(&self) -> String;
    fn func(&self, store: &mut Store<Runtime>) -> Func;
}

dyn_clone::clone_trait_object!(Environment);

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

#[macro_export]
macro_rules! env_runtime {
    ( #[version = $version:literal]
      pub fn $name:ident ( $($args:tt)* ) $(-> $return_values:ty)? { $func:expr }
    ) => {
        #[derive(Clone)]
        pub struct $name;

        impl Environment for $name {
            fn module(&self) -> String {
                let version = stringify!($version);
                String::from("env".to_owned() + version)
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
    #[version = 0]
    pub fn CallContract(
        offset_contract: u32,
        length_contract: u32,
        offset_func_name: u32,
        length_func_name: u32,
        offset_func_args: u32,
        length_func_args: u32
    ) -> i32 {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return RuntimeError::MemoryNotFound as i32,
            };

            // TODO: Depends on the JVM function signature
            // Maybe it will be enough to transmit as bytes
            let contract = match str::from_utf8(
                &memory[offset_contract as usize..offset_contract as usize + length_contract as usize]
            ) {
                Ok(string) => string,
                Err(_) => return RuntimeError::Utf8Error as i32,
            };

            let bytecode = match ctx.stack.jvm_get_bytecode(contract) {
                Ok(bytecode) => bytecode,
                Err(error) => return error.as_i32(),
            };

            let func_name = match str::from_utf8(
                &memory[offset_func_name as usize..offset_func_name as usize + length_func_name as usize]
            ) {
                Ok(string) => string,
                Err(_) => return RuntimeError::Utf8Error as i32,
            };

            let mut input_data: Vec<u8> = vec![];
            input_data.extend_from_slice(
                &memory[offset_func_args as usize..offset_func_args as usize + length_func_args as usize]
            );

            match ctx.stack.call(bytecode, func_name, input_data) {
                Ok(_result) => {
                    // TODO: To be able to use modules, the function must return the value of
                    // But since runtime errors can occur, it is necessary to return the error code
                    // Perhaps the result of the execution should be written to memory
                    //
                    // match result[0] {
                    //     Value::I32(value) => value,
                    //     _ => 0,
                    // }
                    0
                },
                Err(error) => error.as_i32(),
            }
        }
    }
}

pub fn get_envs() -> Vec<Box<dyn Environment>> {
    let mut envs: Vec<Box<dyn Environment>> = vec![];

    let call_contract = CallContract;
    envs.push(Box::new(call_contract));

    envs
}
