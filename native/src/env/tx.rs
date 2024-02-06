use crate::{error::RuntimeError, node::Node, runtime::Runtime};
use wasmi::Caller;

pub fn get_tx_sender(mut caller: Caller<Runtime>) -> (i32, i32, i32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    match ctx.vm.get_tx_sender() {
        Ok(result) => crate::env::write_memory(ctx, memory, offset_memory, result),
        Err(error) => (error.as_i32(), 0, 0),
    }
}
