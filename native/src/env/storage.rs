use crate::{
    data_entry::DataEntry,
    error::{ExecutableError, RuntimeError},
    node::Node,
    runtime::Runtime,
};
use wasmi::Caller;

pub fn get_storage_int(
    offset_address: u32,
    length_address: u32,
    offset_key: u32,
    length_key: u32,
    mut caller: Caller<Runtime>,
) -> (i32, i64) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0),
    };

    let address = if length_address != 0 {
        memory[offset_address as usize..offset_address as usize + length_address as usize].to_vec()
    } else {
        ctx.vm.top_frame().contract_id()
    };

    let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];

    match ctx.vm.get_storage(address.as_slice(), key) {
        Ok(bytes) => match DataEntry::deserialize_storage(bytes.as_slice()) {
            Ok(DataEntry::Integer(integer)) => (0, integer),
            _ => (ExecutableError::FailedDeserializeDataEntry as i32, 0),
        },
        Err(error) => (error.as_i32(), 0),
    }
}

pub fn get_storage_bool(
    offset_address: u32,
    length_address: u32,
    offset_key: u32,
    length_key: u32,
    mut caller: Caller<Runtime>,
) -> (i32, i32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0),
    };

    let address = if length_address != 0 {
        memory[offset_address as usize..offset_address as usize + length_address as usize].to_vec()
    } else {
        ctx.vm.top_frame().contract_id()
    };

    let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];

    match ctx.vm.get_storage(address.as_slice(), key) {
        Ok(bytes) => match DataEntry::deserialize_storage(bytes.as_slice()) {
            Ok(DataEntry::Boolean(boolean)) => (0, boolean),
            _ => (ExecutableError::FailedDeserializeDataEntry as i32, 0),
        },
        Err(error) => (error.as_i32(), 0),
    }
}

pub fn get_storage_binary(
    offset_address: u32,
    length_address: u32,
    offset_key: u32,
    length_key: u32,
    mut caller: Caller<Runtime>,
) -> (i32, i32, i32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let address = if length_address != 0 {
        memory[offset_address as usize..offset_address as usize + length_address as usize].to_vec()
    } else {
        ctx.vm.top_frame().contract_id()
    };

    let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];

    match ctx.vm.get_storage(address.as_slice(), key) {
        Ok(bytes) => {
            let result = match DataEntry::deserialize_storage(bytes.as_slice()) {
                Ok(DataEntry::Binary(bytes)) => bytes,
                _ => return (ExecutableError::FailedDeserializeDataEntry as i32, 0, 0),
            };
            crate::env::write_memory(ctx, memory, offset_memory, result)
        }
        Err(error) => (error.as_i32(), 0, 0),
    }
}

pub fn get_storage_string(
    offset_address: u32,
    length_address: u32,
    offset_key: u32,
    length_key: u32,
    mut caller: Caller<Runtime>,
) -> (i32, i32, i32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let address = if length_address != 0 {
        memory[offset_address as usize..offset_address as usize + length_address as usize].to_vec()
    } else {
        ctx.vm.top_frame().contract_id()
    };

    let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];

    match ctx.vm.get_storage(address.as_slice(), key) {
        Ok(bytes) => {
            let result = match DataEntry::deserialize_storage(bytes.as_slice()) {
                Ok(DataEntry::String(bytes)) => bytes,
                _ => return (ExecutableError::FailedDeserializeDataEntry as i32, 0, 0),
            };
            crate::env::write_memory(ctx, memory, offset_memory, result)
        }
        Err(error) => (error.as_i32(), 0, 0),
    }
}

pub fn set_storage_int(
    offset_key: u32,
    length_key: u32,
    value: i64,
    mut caller: Caller<Runtime>,
) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound as i32,
    };

    let contract_id = ctx.vm.top_frame().contract_id();
    let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];
    let data_entry = DataEntry::Integer(value).serialize(Some(key));

    match ctx
        .vm
        .set_storage(contract_id.as_slice(), data_entry.as_slice())
    {
        Ok(_) => 0,
        Err(error) => error.as_i32(),
    }
}

pub fn set_storage_bool(
    offset_key: u32,
    length_key: u32,
    value: i32,
    mut caller: Caller<Runtime>,
) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound as i32,
    };

    let contract_id = ctx.vm.top_frame().contract_id();
    let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];
    let data_entry = DataEntry::Boolean(value).serialize(Some(key));

    match ctx
        .vm
        .set_storage(contract_id.as_slice(), data_entry.as_slice())
    {
        Ok(_) => 0,
        Err(error) => error.as_i32(),
    }
}

pub fn set_storage_binary(
    offset_key: u32,
    length_key: u32,
    offset_value: u32,
    length_value: u32,
    mut caller: Caller<Runtime>,
) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound as i32,
    };

    let contract_id = ctx.vm.top_frame().contract_id();
    let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];
    let value = &memory[offset_value as usize..offset_value as usize + length_value as usize];
    let data_entry = DataEntry::Binary(value.to_vec()).serialize(Some(key));

    match ctx
        .vm
        .set_storage(contract_id.as_slice(), data_entry.as_slice())
    {
        Ok(_) => 0,
        Err(error) => error.as_i32(),
    }
}

pub fn set_storage_string(
    offset_key: u32,
    length_key: u32,
    offset_value: u32,
    length_value: u32,
    mut caller: Caller<Runtime>,
) -> i32 {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return RuntimeError::MemoryNotFound as i32,
    };

    let contract_id = ctx.vm.top_frame().contract_id();
    let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];
    let value = &memory[offset_value as usize..offset_value as usize + length_value as usize];
    let data_entry = DataEntry::String(value.to_vec()).serialize(Some(key));

    match ctx
        .vm
        .set_storage(contract_id.as_slice(), data_entry.as_slice())
    {
        Ok(_) => 0,
        Err(error) => error.as_i32(),
    }
}
