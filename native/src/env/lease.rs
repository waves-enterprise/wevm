use crate::{error::RuntimeError, node::Node, runtime::Runtime, write_memory};
use wasmi::Caller;

pub fn lease(
    version: u32,
    offset: u32,
    length: u32,
    amount: i64,
    mut caller: Caller<Runtime>,
) -> (i32, i32, i32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let contract_id = ctx.vm.top_frame().contract_id();
    let value = &memory[offset as usize..offset as usize + length as usize];

    let bytes = if version == 1 {
        value.to_vec()
    } else {
        let mut result: Vec<u8> = vec![2];
        match ctx.vm.get_chain_id() {
            Ok(chain_id) => result.push(chain_id as u8),
            Err(error) => return (error.as_i32(), 0, 0),
        }
        result.extend_from_slice(value);
        result
    };

    match ctx
        .vm
        .lease(contract_id.as_slice(), bytes.as_slice(), amount)
    {
        Ok(result) => write_memory!(ctx, memory, offset_memory, result),
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
