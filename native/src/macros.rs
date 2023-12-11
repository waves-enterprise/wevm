/// Macro allows you to wrap imported functions into convenient constructs for linking with `wasmi`.
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

/// Collects Environments into a Vec.
#[macro_export]
macro_rules! env_items {
    ( $($env:ident),+ ) => {
        pub fn to_vec() -> Vec<Box<dyn Environment>> {
            let mut result: Vec<Box<dyn Environment>> = vec![];

            $(
                let env = $env;
                result.push(Box::new(env));
            )+

            result
        }
    }
}

/// Wrapper over writing to WASM linear memory.
/// Functions using this wrapper return (i32, i32, i32):
/// * First value - error code
/// * Second value - memory offset
/// * Third value - length of data in memory
#[macro_export]
macro_rules! write_memory {
    ($ctx:expr, $memory:expr, $offset_memory:expr, $result:expr) => {{
        let length = $result.len();
        $memory[$offset_memory..$offset_memory + length].copy_from_slice($result.as_slice());
        $ctx.set_heap_base(($offset_memory + length) as i32);
        (0, $offset_memory as i32, length as i32)
    }};
}
