use crate::{
    env::Environment,
    env_items, env_runtime,
    jvm::Jvm,
    runtime::{Runtime, RuntimeError},
    write_memory,
};
use convert_case::{Case, Casing};
use wasmi::{Caller, Func, Store};

env_items!(GetTxSender);

env_runtime! {
    #[version = 0]
    pub fn GetTxSender() -> (i32, i32, i32) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
            };
            let offset_memory = ctx.heap_base() as usize;

            match ctx.stack.get_tx_sender() {
                Ok(result) => write_memory!(ctx, memory, offset_memory, result),
                Err(error) => (error.as_i32(), 0, 0),
            }
        }
    }
}
