/// Macro allows you to wrap imported functions into convenient constructs for linking with `wasmi`.
#[macro_export]
macro_rules! module {
    (
        #[version = $version:literal]
        $( fn $name:ident ( $($args:tt)* ) $(-> $return_values:ty)? { $func:expr } )+
    ) => {
        pub fn modules() -> Vec<Module> {
            let mut vec: Vec<Module> = vec![];

            $(
                fn $name(store: &mut Store<Runtime>) -> (String, String, Func) {
                    let version = stringify!($version);
                    let module = String::from("env".to_owned() + version);

                    let name = stringify!($name);

                    (module, name.to_string(), Func::wrap(
                        store,
                        |caller: Caller<Runtime>, $($args)*| $(-> $return_values)? {
                            $func(caller)
                        }
                    ))
                }

                vec.push($name);
            )+

            vec
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
