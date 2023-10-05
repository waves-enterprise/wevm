use crate::{env::Environment, env_items, env_runtime, jvm::Jvm, runtime::Runtime};
use convert_case::{Case, Casing};
use wasmi::{Caller, Func, Store};

env_items!(GetBlockTimestamp, GetBlockHeight);

env_runtime! {
    #[version = 0]
    pub fn GetBlockTimestamp() -> (i32, i64) {
        |caller: Caller<Runtime>| {
            match caller.data().stack.get_block_timestamp() {
                Ok(result) => (0, result),
                Err(error) => (error.as_i32(), 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn GetBlockHeight() -> (i32, i64) {
        |caller: Caller<Runtime>| {
            match caller.data().stack.get_block_height() {
                Ok(result) => (0, result),
                Err(error) => (error.as_i32(), 0),
            }
        }
    }
}
