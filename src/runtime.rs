use wasmi::{Func, Memory, Store};

pub trait Runtime {
    fn new() -> Self;
    fn memory(&self) -> Option<Memory>;
    fn set_memory(&mut self, memory: Memory);
}

pub trait Environment<T> {
    fn module(&self) -> String;
    fn name(&self) -> String;
    fn func(&self, store: &mut Store<T>) -> Func;
}

#[derive(Clone, Copy)]
pub struct HostRuntime {
    memory: Option<Memory>,
}

impl Runtime for HostRuntime {
    fn new() -> Self {
        HostRuntime { memory: None }
    }

    fn memory(&self) -> Option<Memory> {
        self.memory
    }

    fn set_memory(&mut self, memory: Memory) {
        self.memory = Some(memory);
    }
}

impl HostRuntime {
    pub fn get_storage(&self) {
        panic!("Not implimented!");
    }
}

#[macro_export]
macro_rules! env_runtime {
    ( pub fn $name:ident <$runtime:ty> ( $($args:tt)* ) $(-> $return_values:ty)? { $func:expr } ) => {
        pub struct $name;

        impl Environment<$runtime> for $name {
            fn module(&self) -> String {
                String::from("env")
            }

            fn name(&self) -> String {
                let name = stringify!($name);
                name.from_case(Case::Pascal).to_case(Case::Snake)
            }

            fn func(&self, store: &mut Store<$runtime>) -> Func {
                Func::wrap(
                    store,
                    |caller: Caller<$runtime>, $($args)*| $(-> $return_values)? {
                        $func(caller)
                    }
                )
            }
        }
    }
}
