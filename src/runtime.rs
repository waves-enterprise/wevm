use wasmi::{Func, Memory, Store};

pub trait Environment {
    fn module(&self) -> String;
    fn name(&self) -> String;
    fn func(&self, store: &mut Store<Runtime>) -> Func;
}

pub struct Runtime {
    memory: Option<Memory>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime { memory: None }
    }

    pub fn memory(&self) -> Option<Memory> {
        self.memory
    }

    pub fn set_memory(&mut self, memory: Memory) {
        self.memory = Some(memory);
    }

    pub fn get_contract(&self) -> Vec<u8> {
        panic!("Not implimented!");
    }
}

#[macro_export]
macro_rules! env_runtime {
    ( pub fn $name:ident ( $($args:tt)* ) $(-> $return_values:ty)? { $func:expr } ) => {
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
