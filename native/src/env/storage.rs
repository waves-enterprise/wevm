use crate::{
    data_entry::DataEntry,
    env::Environment,
    env_items, env_runtime,
    exec::ExecutableError,
    jvm::Jvm,
    runtime::{Runtime, RuntimeError},
    write_memory,
};
use convert_case::{Case, Casing};
use wasmi::{Caller, Func, Store};

env_items!(
    GetStorageInt,
    GetStorageBool,
    GetStorageBinary,
    GetStorageString,
    SetStorageInt,
    SetStorageBool,
    SetStorageBinary,
    SetStorageString
);

env_runtime! {
    #[version = 0]
    pub fn GetStorageInt(
        offset_address: u32,
        length_address: u32,
        offset_key: u32,
        length_key: u32,
    ) -> (i32, i64) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0),
            };

            let address = if length_address != 0 {
                memory[offset_address as usize..offset_address as usize + length_address as usize].to_vec()
            } else {
                ctx.stack.top_frame().contract_id()
            };

            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];

            match ctx.stack.get_storage(address.as_slice(), key) {
                Ok(bytes) => {
                    match DataEntry::deserialize_storage(bytes.as_slice()) {
                        Ok(DataEntry::Integer(integer)) => (0, integer),
                        _ => (ExecutableError::FailedDeserializeDataEntry as i32, 0),
                    }
                },
                Err(error) => (error.as_i32(), 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn GetStorageBool(
        offset_address: u32,
        length_address: u32,
        offset_key: u32,
        length_key: u32,
    ) -> (i32, i32) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0),
            };

            let address = if length_address != 0 {
                memory[offset_address as usize..offset_address as usize + length_address as usize].to_vec()
            } else {
                ctx.stack.top_frame().contract_id()
            };

            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];

            match ctx.stack.get_storage(address.as_slice(), key) {
                Ok(bytes) => {
                    match DataEntry::deserialize_storage(bytes.as_slice()) {
                        Ok(DataEntry::Boolean(boolean)) => (0, boolean),
                        _ => (ExecutableError::FailedDeserializeDataEntry as i32, 0),
                    }

                },
                Err(error) => (error.as_i32(), 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn GetStorageBinary(
        offset_address: u32,
        length_address: u32,
        offset_key: u32,
        length_key: u32,
    ) -> (i32, i32, i32) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
            };
            let offset_memory = ctx.heap_base() as usize;

            let address = if length_address != 0 {
                memory[offset_address as usize..offset_address as usize + length_address as usize].to_vec()
            } else {
                ctx.stack.top_frame().contract_id()
            };

            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];

            match ctx.stack.get_storage(address.as_slice(), key) {
                Ok(bytes) => {
                    let result = match DataEntry::deserialize_storage(bytes.as_slice()) {
                        Ok(DataEntry::Binary(bytes)) => bytes,
                        _ => return (ExecutableError::FailedDeserializeDataEntry as i32, 0, 0),
                    };
                    write_memory!(ctx, memory, offset_memory, result)
                },
                Err(error) => (error.as_i32(), 0, 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn GetStorageString(
        offset_address: u32,
        length_address: u32,
        offset_key: u32,
        length_key: u32,
    ) -> (i32, i32, i32) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
            };
            let offset_memory = ctx.heap_base() as usize;

            let address = if length_address != 0 {
                memory[offset_address as usize..offset_address as usize + length_address as usize].to_vec()
            } else {
                ctx.stack.top_frame().contract_id()
            };

            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];

            match ctx.stack.get_storage(address.as_slice(), key) {
                Ok(bytes) => {
                    let result = match DataEntry::deserialize_storage(bytes.as_slice()) {
                        Ok(DataEntry::String(bytes)) => bytes,
                        _ => return (ExecutableError::FailedDeserializeDataEntry as i32, 0, 0),
                    };
                    write_memory!(ctx, memory, offset_memory, result)
                },
                Err(error) => (error.as_i32(), 0, 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn SetStorageInt(
        offset_key: u32,
        length_key: u32,
        value: i64,
    ) -> i32 {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return RuntimeError::MemoryNotFound as i32,
            };

            let contract_id = ctx.stack.top_frame().contract_id();
            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];
            let data_entry = DataEntry::Integer(value).serialize(Some(key));

            match ctx.stack.set_storage(contract_id.as_slice(), data_entry.as_slice()) {
                Ok(_) => 0,
                Err(error) => error.as_i32(),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn SetStorageBool(
        offset_key: u32,
        length_key: u32,
        value: i32,
    ) -> i32 {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return RuntimeError::MemoryNotFound as i32,
            };

            let contract_id = ctx.stack.top_frame().contract_id();
            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];
            let data_entry = DataEntry::Boolean(value).serialize(Some(key));

            match ctx.stack.set_storage(contract_id.as_slice(), data_entry.as_slice()) {
                Ok(_) => 0,
                Err(error) => error.as_i32(),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn SetStorageBinary(
        offset_key: u32,
        length_key: u32,
        offset_value: u32,
        length_value: u32,
    ) -> i32 {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return RuntimeError::MemoryNotFound as i32,
            };

            let contract_id = ctx.stack.top_frame().contract_id();
            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];
            let value = &memory[offset_value as usize..offset_value as usize + length_value as usize];
            let data_entry = DataEntry::Binary(value.to_vec()).serialize(Some(key));

            match ctx.stack.set_storage(contract_id.as_slice(), data_entry.as_slice()) {
                Ok(_) => 0,
                Err(error) => error.as_i32(),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn SetStorageString(
        offset_key: u32,
        length_key: u32,
        offset_value: u32,
        length_value: u32,
    ) -> i32 {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return RuntimeError::MemoryNotFound as i32,
            };

            let contract_id = ctx.stack.top_frame().contract_id();
            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];
            let value = &memory[offset_value as usize..offset_value as usize + length_value as usize];
            let data_entry = DataEntry::String(value.to_vec()).serialize(Some(key));

            match ctx.stack.set_storage(contract_id.as_slice(), data_entry.as_slice()) {
                Ok(_) => 0,
                Err(error) => error.as_i32(),
            }
        }
    }
}
