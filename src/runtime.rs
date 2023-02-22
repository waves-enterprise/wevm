use crate::stack::Stack;
use convert_case::{Case, Casing};
use dyn_clone::DynClone;
use std::str;
use wasmi::{Caller, Func, Memory, Store};

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
}

#[macro_export]
macro_rules! env_runtime {
    ( pub fn $name:ident ( $($args:tt)* ) $(-> $return_values:ty)? { $func:expr } ) => {
        #[derive(Clone)]
        pub struct $name;

        impl Environment for $name {
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

            let func_name = str::from_utf8(&memory[offset_func_name as usize..offset_func_name as usize + length_func_name as usize])
                .expect("Error converts a slice of bytes to a string slice");

            // TODO: Parse args
            let func_args: [String; 0] = [];

            let bytecode = ctx.get_bytecode(contract_name);

            ctx.call_contract(bytecode, func_name, &func_args)
        }
    }
}
