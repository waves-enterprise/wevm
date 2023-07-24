use crate::{
    jvm::Jvm,
    runtime::{Runtime, RuntimeError},
};
use convert_case::{Case, Casing};
use dyn_clone::DynClone;
use std::str;
use wasmi::{Caller, Func, Store};

pub trait Environment: DynClone {
    fn module(&self) -> String;
    fn name(&self) -> String;
    fn func(&self, store: &mut Store<Runtime>) -> Func;
}

dyn_clone::clone_trait_object!(Environment);

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

pub fn envs() -> Vec<Box<dyn Environment>> {
    let call_contract = CallContract;

    vec![Box::new(call_contract)]
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
