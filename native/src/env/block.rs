use crate::{node::Node, runtime::Runtime};
use wasmi::Caller;

pub fn get_block_timestamp(caller: Caller<Runtime>) -> (i32, i64) {
    match caller.data().vm.get_block_timestamp() {
        Ok(result) => (0, result),
        Err(error) => (error.as_i32(), 0),
    }
}

pub fn get_block_height(caller: Caller<Runtime>) -> (i32, i64) {
    match caller.data().vm.get_block_height() {
        Ok(result) => (0, result),
        Err(error) => (error.as_i32(), 0),
    }
}
