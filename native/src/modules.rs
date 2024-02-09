mod v0;
mod v1;

use crate::runtime::Runtime;
use wasmi::{Func, Store};

pub type Module = fn(&mut Store<Runtime>) -> (String, String, Func);

pub fn all() -> Vec<Module> {
    let mut vec = vec![];
    vec.extend(v0::modules());
    vec.extend(v1::modules());
    vec
}

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
