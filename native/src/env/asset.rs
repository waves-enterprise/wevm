use crate::{error::RuntimeError, node::Node, runtime::Runtime};
use log::error;
use wasmi::Caller;

pub fn get_balance(
    offset_asset_id: u32,
    length_asset_id: u32,
    offset_asset_holder: u32,
    length_asset_holder: u32,
    type_: u32,
    version: u32,
    mut caller: Caller<Runtime>,
) -> (i32, i64) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound.as_i32(), 0),
    };

    let asset_id =
        &memory[offset_asset_id as usize..offset_asset_id as usize + length_asset_id as usize];

    let (type_, bytes) = if length_asset_holder != 0 {
        let bytes = &memory[offset_asset_holder as usize
            ..offset_asset_holder as usize + length_asset_holder as usize];
        (type_, bytes.to_vec())
    } else {
        (1, ctx.vm.top_frame().contract_id())
    };

    let asset_holder = match crate::env::get_asset_holder(ctx, type_, version, bytes) {
        Ok(bytes) => bytes,
        Err(error) => return (error.as_i32(), 0),
    };

    match ctx.vm.get_balance(asset_id, asset_holder.as_slice()) {
        Ok(result) => (0, result),
        Err(error) => {
            error!("{}", error);
            (error.as_i32(), 0)
        }
    }
}

pub fn transfer(
    offset_asset_id: u32,
    length_asset_id: u32,
    offset_recipient: u32,
    length_recipient: u32,
    type_: u32,
    version: u32,
    amount: i64,
    mut caller: Caller<Runtime>,
) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound.as_i32(),
    };

    let contract_id = ctx.vm.top_frame().contract_id();
    let asset_id =
        &memory[offset_asset_id as usize..offset_asset_id as usize + length_asset_id as usize];

    let recipient =
        &memory[offset_recipient as usize..offset_recipient as usize + length_recipient as usize];
    let asset_holder = match crate::env::get_asset_holder(ctx, type_, version, recipient.to_vec()) {
        Ok(bytes) => bytes,
        Err(error) => {
            error!("{}", error);
            return error.as_i32();
        }
    };

    match ctx.vm.transfer(
        contract_id.as_slice(),
        asset_id,
        asset_holder.as_slice(),
        amount,
    ) {
        Ok(_) => 0,
        Err(error) => {
            error!("{}", error);
            error.as_i32()
        }
    }
}

pub fn issue(
    offset_name: u32,
    length_name: u32,
    offset_description: u32,
    length_description: u32,
    quantity: i64,
    decimals: i64,
    is_reissuable: i32,
    mut caller: Caller<Runtime>,
) -> (i32, u32, u32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound.as_i32(), 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let contract_id = ctx.vm.top_frame().contract_id();
    let name = &memory[offset_name as usize..offset_name as usize + length_name as usize];
    let description = &memory
        [offset_description as usize..offset_description as usize + length_description as usize];

    match ctx.vm.issue(
        contract_id.as_slice(),
        name,
        description,
        quantity,
        decimals,
        is_reissuable != 0,
    ) {
        Ok(result) => crate::env::write_memory(ctx, memory, offset_memory, result),
        Err(error) => {
            error!("{}", error);
            (error.as_i32(), 0, 0)
        }
    }
}

pub fn burn(
    offset_asset_id: u32,
    length_asset_id: u32,
    amount: i64,
    mut caller: Caller<Runtime>,
) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => {
            return RuntimeError::MemoryNotFound.as_i32();
        }
    };

    let contract_id = ctx.vm.top_frame().contract_id();
    let asset_id =
        &memory[offset_asset_id as usize..offset_asset_id as usize + length_asset_id as usize];

    match ctx.vm.burn(contract_id.as_slice(), asset_id, amount) {
        Ok(_) => 0,
        Err(error) => {
            error!("{}", error);
            error.as_i32()
        }
    }
}

pub fn reissue(
    offset_asset_id: u32,
    length_asset_id: u32,
    amount: i64,
    is_reissuable: i32,
    mut caller: Caller<Runtime>,
) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound.as_i32(),
    };

    let contract_id = ctx.vm.top_frame().contract_id();
    let asset_id =
        &memory[offset_asset_id as usize..offset_asset_id as usize + length_asset_id as usize];

    match ctx
        .vm
        .reissue(contract_id.as_slice(), asset_id, amount, is_reissuable != 0)
    {
        Ok(_) => 0,
        Err(error) => {
            error!("{}", error);
            error.as_i32()
        }
    }
}
