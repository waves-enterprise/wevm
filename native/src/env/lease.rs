use crate::{
    env::Environment,
    env_items, env_runtime,
    jvm::Jvm,
    runtime::{Runtime, RuntimeError},
    write_memory,
};
use convert_case::{Case, Casing};
use wasmi::{Caller, Func, Store};

env_items!(Lease, CancelLease);

env_runtime! {
    #[version = 0]
    pub fn Lease(
        offset_recipient: u32,
        length_recipient: u32,
        amount: i64,
    ) -> (i32, i32, i32) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
            };
            let offset_memory = ctx.heap_base() as usize;

            let contract_id = ctx.stack.top_frame().contract_id();
            let recipient = &memory[offset_recipient as usize..offset_recipient as usize + length_recipient as usize];

            match ctx.stack.lease(contract_id.as_slice(), recipient, amount) {
                Ok(result) => write_memory!(ctx, memory, offset_memory, result),
                Err(error) => (error.as_i32(), 0, 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn CancelLease(
        offset_lease_id: u32,
        length_lease_id: u32,
    ) -> i32 {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return RuntimeError::MemoryNotFound as i32,
            };

            let contract_id = ctx.stack.top_frame().contract_id();
            let lease_id = &memory[offset_lease_id as usize..offset_lease_id as usize + length_lease_id as usize];

            match ctx.stack.cancel_lease(contract_id.as_slice(), lease_id) {
                Ok(_) => 0,
                Err(error) => error.as_i32(),
            }
        }
    }
}
