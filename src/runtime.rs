use convert_case::{Case, Casing};
use dyn_clone::DynClone;
use wasmi::{Caller, Func, Memory, Store};

use crate::stack::Stack;

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
