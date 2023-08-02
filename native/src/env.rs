use crate::{
    jvm::Jvm,
    runtime::{Runtime, RuntimeError},
};
use convert_case::{Case, Casing};
use dyn_clone::DynClone;
use std::str;
use wasmi::{core::Value, Caller, Func, Store};

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
    let get_storage_i64 = GetStorageInt;

    vec![Box::new(call_contract), Box::new(get_storage_i64)]
}

env_runtime! {
    #[version = 0]
    pub fn CallContract(
        offset_contract_id: u32,
        length_contract_id: u32,
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

            let contract_id = &memory[offset_contract_id as usize..offset_contract_id as usize + length_contract_id as usize];

            let bytecode = match ctx.stack.get_bytecode(contract_id) {
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
                Ok(result) => {
                    // TODO: Functions cannot return any values, they can only return an error code
                    if result.len() != 1 {
                        return RuntimeError::InvalidResult as i32;
                    }

                    match result[0] {
                        Value::I32(value) => value,
                        _ => RuntimeError::InvalidResult as i32,
                    }
                },
                Err(error) => error.as_i32(),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn GetStorageInt(
        offset_contract_id: u32,
        length_contract_id: u32,
        offset_key: u32,
        length_key: u32,
    ) -> (i32, i64) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0),
            };

            let contract_id = &memory[offset_contract_id as usize..offset_contract_id as usize + length_contract_id as usize];
            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];

            match ctx.stack.get_storage(contract_id, key) {
                Ok(result) => {
                    if result.len() == 8 {
                        let mut temp = [0u8; 8];
                        temp.copy_from_slice(result.as_slice());
                        (0, i64::from_be_bytes(temp))
                    } else {
                        (RuntimeError::InvalidInteger as i32, 0)
                    }
                },
                Err(error) => (error.as_i32(), 0),
            }
        }
    }
}
