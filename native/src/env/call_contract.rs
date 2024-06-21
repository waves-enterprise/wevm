use crate::{
    error::RuntimeError,
    node::Node,
    runtime::{data_entry::DataEntry, payment_id::PaymentId, Runtime},
};
use log::error;
use std::str;
use wasmi::{Caller, Value};

pub fn call_arg_int(value: i64, mut caller: Caller<Runtime>) {
    caller.data_mut().params.push(DataEntry::Integer(value));
}

pub fn call_arg_bool(value: i32, mut caller: Caller<Runtime>) {
    caller.data_mut().params.push(DataEntry::Boolean(value));
}

pub fn call_arg_binary(offset_value: u32, length_value: u32, mut caller: Caller<Runtime>) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound.as_i32(),
    };

    let value = &memory[offset_value as usize..offset_value as usize + length_value as usize];
    ctx.params.push(DataEntry::Binary(value.to_vec()));

    0
}

pub fn call_arg_string(offset_value: u32, length_value: u32, mut caller: Caller<Runtime>) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound.as_i32(),
    };

    let value = &memory[offset_value as usize..offset_value as usize + length_value as usize];
    ctx.params.push(DataEntry::String(value.to_vec()));

    0
}

pub fn call_payment(
    offset_asset_id: u32,
    length_asset_id: u32,
    amount: i64,
    mut caller: Caller<Runtime>,
) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound.as_i32(),
    };

    let asset_id =
        &memory[offset_asset_id as usize..offset_asset_id as usize + length_asset_id as usize];
    ctx.payments.push(asset_id, amount);

    0
}

pub fn call_contract(
    offset_contract_id: u32,
    length_contract_id: u32,
    offset_func_name: u32,
    length_func_name: u32,
    offset_params: Option<u32>,
    length_params: Option<u32>,
    mut caller: Caller<Runtime>,
) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound.as_i32(),
    };

    let callable_contract_id = &memory
        [offset_contract_id as usize..offset_contract_id as usize + length_contract_id as usize];

    let bytecode = match ctx.vm.get_bytecode(callable_contract_id) {
        Ok(bytecode) => bytecode,
        Err(error) => {
            error!("{}", error);
            return error.as_i32();
        }
    };

    let func_name = match str::from_utf8(
        &memory[offset_func_name as usize..offset_func_name as usize + length_func_name as usize],
    ) {
        Ok(string) => string,
        Err(_) => return RuntimeError::Utf8Error.as_i32(),
    };

    let params: Vec<u8> = match (offset_params, length_params) {
        (Some(offset), Some(length)) => {
            memory[offset as usize..offset as usize + length as usize].to_vec()
        }
        _ => {
            let bytes = ctx.params.as_bytes();
            ctx.params.reset();
            bytes
        }
    };

    // Since a single contract can be invoked multiple times during execution,
    // it is necessary to have a unique identifier to distinguish each unique execution
    let nonce = ctx.vm.get_nonce();
    let payment_id = PaymentId::new(callable_contract_id.to_vec(), nonce);

    let self_contract_id = ctx.vm.top_frame().contract_id();

    if !ctx.payments.is_empty() {
        let payments = ctx.payments.as_bytes();
        ctx.payments.reset();

        match ctx.vm.add_payments(
            self_contract_id.as_slice(),
            payment_id.as_bytes().as_slice(),
            &payments,
        ) {
            Ok(()) => (),
            Err(error) => {
                error!("{}", error);
                return error.as_i32();
            }
        }
    }

    match ctx.vm.call(
        callable_contract_id.to_vec(),
        bytecode,
        nonce,
        func_name,
        &params,
    ) {
        Ok(result) => {
            // TODO: Functions cannot return any values, they can only return an error code
            let error = RuntimeError::InvalidResult(format!("Functions cannot return any values, they can only return an error code. Result: {:?}", result));
            if result.len() != 1 {
                error!("{}", error);
                return error.as_i32();
            }

            match result[0] {
                Value::I32(value) => value,
                _ => error.as_i32(),
            }
        }
        Err(error) => {
            error!("{}", error);
            error.as_i32()
        }
    }
}
