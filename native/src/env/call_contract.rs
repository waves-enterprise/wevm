use crate::{
    data_entry::DataEntry, error::RuntimeError, node::Node, runtime::Runtime, vm::create_payment_id,
};
use std::str;
use wasmi::{core::Value, Caller};

pub fn call_arg_int(value: i64, mut caller: Caller<Runtime>) {
    caller.data_mut().args.push(DataEntry::Integer(value));
}

pub fn call_arg_bool(value: i32, mut caller: Caller<Runtime>) {
    caller.data_mut().args.push(DataEntry::Boolean(value));
}

pub fn call_arg_binary(offset_value: u32, length_value: u32, mut caller: Caller<Runtime>) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound as i32,
    };

    let value = &memory[offset_value as usize..offset_value as usize + length_value as usize];
    ctx.args.push(DataEntry::Binary(value.to_vec()));

    0
}

pub fn call_arg_string(offset_value: u32, length_value: u32, mut caller: Caller<Runtime>) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound as i32,
    };

    let value = &memory[offset_value as usize..offset_value as usize + length_value as usize];
    ctx.args.push(DataEntry::String(value.to_vec()));

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
        None => return RuntimeError::MemoryNotFound as i32,
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
    mut caller: Caller<Runtime>,
) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound as i32,
    };

    let callable_contract_id = &memory
        [offset_contract_id as usize..offset_contract_id as usize + length_contract_id as usize];

    let bytecode = match ctx.vm.get_bytecode(callable_contract_id) {
        Ok(bytecode) => bytecode,
        Err(error) => return error.as_i32(),
    };

    let func_name = match str::from_utf8(
        &memory[offset_func_name as usize..offset_func_name as usize + length_func_name as usize],
    ) {
        Ok(string) => string,
        Err(_) => return RuntimeError::Utf8Error as i32,
    };

    let (input_data, payments) = ctx.args_and_payments();

    // Since a single contract can be invoked multiple times during execution,
    // it is necessary to have a unique identifier to distinguish each unique execution
    let nonce = ctx.vm.get_nonce();
    let payment_id = create_payment_id(callable_contract_id.to_vec(), nonce);

    let self_contract_id = ctx.vm.top_frame().contract_id();
    match ctx.vm.add_payments(
        self_contract_id.as_slice(),
        payment_id.as_slice(),
        &payments,
    ) {
        Ok(()) => (),
        Err(error) => return error.as_i32(),
    }

    match ctx.vm.call(
        callable_contract_id.to_vec(),
        bytecode,
        nonce,
        func_name,
        input_data,
    ) {
        Ok(result) => {
            // TODO: Functions cannot return any values, they can only return an error code
            if result.len() != 1 {
                return RuntimeError::InvalidResult as i32;
            }

            match result[0] {
                Value::I32(value) => value,
                _ => RuntimeError::InvalidResult as i32,
            }
        }
        Err(error) => error.as_i32(),
    }
}
