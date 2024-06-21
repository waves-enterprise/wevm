use crate::{env::Field, error::RuntimeError, node::Node, runtime::Runtime};
use log::error;
use wasmi::Caller;

pub fn get_payments(caller: Caller<Runtime>) -> (i32, i64) {
    let payment_id = caller.data().vm.top_frame().payment_id();

    match caller.data().vm.get_tx_payments(payment_id.as_slice()) {
        Ok(result) => (0, result),
        Err(error) => {
            error!("{}", error);
            (error.as_i32(), 0)
        }
    }
}

pub fn get_payment_asset_id(number: i64, mut caller: Caller<Runtime>) -> (i32, u32, u32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound.as_i32(), 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let payment_id = ctx.vm.top_frame().payment_id();

    match ctx
        .vm
        .get_tx_payment_asset_id(payment_id.as_slice(), number)
    {
        Ok(result) => crate::env::write_memory(ctx, memory, offset_memory, result),
        Err(error) => {
            error!("{}", error);
            (error.as_i32(), 0, 0)
        }
    }
}

pub fn get_payment_amount(number: i64, caller: Caller<Runtime>) -> (i32, i64) {
    let payment_id = caller.data().vm.top_frame().payment_id();

    match caller
        .data()
        .vm
        .get_tx_payment_amount(payment_id.as_slice(), number)
    {
        Ok(result) => (0, result),
        Err(error) => {
            error!("{}", error);
            (error.as_i32(), 0)
        }
    }
}

pub fn tx(field: Field, mut caller: Caller<Runtime>) -> (i32, u32, u32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound.as_i32(), 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let field = match field {
        Field::String(name) => name.into_bytes(),
        Field::Binary(offset, length) => {
            memory[offset as usize..offset as usize + length as usize].to_vec()
        }
    };

    match ctx.vm.tx(field.as_slice()) {
        Ok(result) => crate::env::write_memory(ctx, memory, offset_memory, result),
        Err(error) => {
            error!("{}", error);
            (error.as_i32(), 0, 0)
        }
    }
}
