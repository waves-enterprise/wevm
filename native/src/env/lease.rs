use crate::{error::RuntimeError, node::Node, runtime::Runtime};
use wasmi::Caller;

pub fn lease(
    offset_recipient: u32,
    length_recipient: u32,
    version: u32,
    amount: i64,
    mut caller: Caller<Runtime>,
) -> (i32, u32, u32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let contract_id = ctx.vm.top_frame().contract_id();
    let bytes =
        &memory[offset_recipient as usize..offset_recipient as usize + length_recipient as usize];

    let asset_holder = match crate::env::get_asset_holder(ctx, 0, version, bytes.to_vec()) {
        Ok(bytes) => bytes,
        Err(error) => return (error.as_i32(), 0, 0),
    };

    match ctx
        .vm
        .lease(contract_id.as_slice(), asset_holder.as_slice(), amount)
    {
        Ok(result) => crate::env::write_memory(ctx, memory, offset_memory, result),
        Err(error) => (error.as_i32(), 0, 0),
    }
}

pub fn cancel_lease(
    offset_lease_id: u32,
    length_lease_id: u32,
    mut caller: Caller<Runtime>,
) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound as i32,
    };

    let contract_id = ctx.vm.top_frame().contract_id();
    let lease_id =
        &memory[offset_lease_id as usize..offset_lease_id as usize + length_lease_id as usize];

    match ctx.vm.cancel_lease(contract_id.as_slice(), lease_id) {
        Ok(_) => 0,
        Err(error) => error.as_i32(),
    }
}
