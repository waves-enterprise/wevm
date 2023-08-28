use crate::{
    data_entry::DataEntry,
    env_runtime,
    exec::ExecutableError,
    jvm::Jvm,
    runtime::{Runtime, RuntimeError},
    write_memory,
};
use base58::FromBase58;
use convert_case::{Case, Casing};
use dyn_clone::DynClone;
use std::str;
use wasmi::{core::Value, Caller, Func, Store};

pub trait Environment: DynClone {
    fn module(&self) -> String;
    fn name(&self) -> String;
    fn func(&self, store: &mut Store<Runtime>) -> Func;
}

dyn_clone::clone_trait_object!(Environment);

pub fn envs() -> Vec<Box<dyn Environment>> {
    let base_58 = Base58;
    let call_contract = CallContract;
    let get_storage_int = GetStorageInt;
    let get_storage_bool = GetStorageBool;
    let get_storage_binary = GetStorageBinary;
    let get_storage_string = GetStorageString;
    let set_storage_int = SetStorageInt;
    let set_storage_bool = SetStorageBool;
    let set_storage_binary = SetStorageBinary;
    let set_storage_string = SetStorageString;
    let get_balance = GetBalance;
    let transfer = Transfer;
    let issue = Issue;
    let burn = Burn;
    let reissue = Reissue;
    let lease = Lease;
    let cancel_lease = CancelLease;
    let get_block_timestamp = GetBlockTimestamp;
    let get_block_height = GetBlockHeight;
    let get_tx_sender = GetTxSender;
    let get_tx_payments = GetTxPayments;
    let get_tx_payment_asset_id = GetTxPaymentAssetId;
    let get_tx_payment_amount = GetTxPaymentAmount;

    vec![
        Box::new(base_58),
        Box::new(call_contract),
        Box::new(get_storage_int),
        Box::new(get_storage_bool),
        Box::new(get_storage_binary),
        Box::new(get_storage_string),
        Box::new(set_storage_int),
        Box::new(set_storage_bool),
        Box::new(set_storage_binary),
        Box::new(set_storage_string),
        Box::new(get_balance),
        Box::new(transfer),
        Box::new(issue),
        Box::new(burn),
        Box::new(reissue),
        Box::new(lease),
        Box::new(cancel_lease),
        Box::new(get_block_timestamp),
        Box::new(get_block_height),
        Box::new(get_tx_sender),
        Box::new(get_tx_payments),
        Box::new(get_tx_payment_asset_id),
        Box::new(get_tx_payment_amount),
    ]
}

env_runtime! {
    #[version = 0]
    pub fn Base58(
        offset_bytes: u32,
        length_bytes: u32,
    ) -> (i32, i32, i32) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
            };
            let offset_memory = ctx.heap_base() as usize;

            let value = match str::from_utf8(
                &memory[offset_bytes as usize..offset_bytes as usize + length_bytes as usize]
            ) {
                Ok(string) => string,
                Err(_) => return (RuntimeError::Utf8Error as i32, 0 , 0),
            };

            match value.from_base58() {
                Ok(result) => write_memory!(ctx, memory, offset_memory, result),
                Err(_) => (RuntimeError::Base58Error as i32, 0, 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn CallContract(
        offset_contract_id: u32,
        length_contract_id: u32,
        offset_func_name: u32,
        length_func_name: u32,
        offset_func_args: u32,
        length_func_args: u32
    ) -> i32 {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return RuntimeError::MemoryNotFound as i32,
            };

            let contract_id = &memory[offset_contract_id as usize..offset_contract_id as usize + length_contract_id as usize];

            let bytecode = match ctx.stack.get_bytecode(contract_id) {
                Ok(bytecode) => bytecode,
                Err(error) => return error.as_i32(),
            };

            let func_name = match str::from_utf8(
                &memory[offset_func_name as usize..offset_func_name as usize + length_func_name as usize]
            ) {
                Ok(string) => string,
                Err(_) => return RuntimeError::Utf8Error as i32,
            };

            let mut input_data: Vec<u8> = vec![];
            input_data.extend_from_slice(
                &memory[offset_func_args as usize..offset_func_args as usize + length_func_args as usize]
            );

            match ctx.stack.call(bytecode, func_name, input_data) {
                Ok(result) => {
                    // TODO: Functions cannot return any values, they can only return an error code
                    if result.len() != 1 {
                        return RuntimeError::InvalidResult as i32;
                    }

                    match result[0] {
                        Value::I32(value) => value,
                        _ => RuntimeError::InvalidResult as i32,
                    }
                },
                Err(error) => error.as_i32(),
            }
        }
    }
}

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

            let address = &memory[offset_address as usize..offset_address as usize + length_address as usize];
            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];

            match ctx.stack.get_storage(address, key) {
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

            let address = &memory[offset_address as usize..offset_address as usize + length_address as usize];
            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];

            match ctx.stack.get_storage(address, key) {
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

            let address = &memory[offset_address as usize..offset_address as usize + length_address as usize];
            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];

            match ctx.stack.get_storage(address, key) {
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

            let address = &memory[offset_address as usize..offset_address as usize + length_address as usize];
            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];

            match ctx.stack.get_storage(address, key) {
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

            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];
            let data_entry = DataEntry::Integer(value).serialize(key);

            match ctx.stack.set_storage(data_entry.as_slice()) {
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

            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];
            let data_entry = DataEntry::Boolean(value).serialize(key);

            match ctx.stack.set_storage(data_entry.as_slice()) {
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

            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];
            let value = &memory[offset_value as usize..offset_value as usize + length_value as usize];
            let data_entry = DataEntry::Binary(value.to_vec()).serialize(key);

            match ctx.stack.set_storage(data_entry.as_slice()) {
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

            let key = &memory[offset_key as usize..offset_key as usize + length_key as usize];
            let value = &memory[offset_value as usize..offset_value as usize + length_value as usize];
            let data_entry = DataEntry::String(value.to_vec()).serialize(key);

            match ctx.stack.set_storage(data_entry.as_slice()) {
                Ok(_) => 0,
                Err(error) => error.as_i32(),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn GetBalance(
        offset_asset_id: u32,
        length_asset_id: u32,
        offset_address: u32,
        length_address: u32,
    ) -> (i32, i64) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0),
            };

            let asset_id = &memory[offset_asset_id as usize..offset_asset_id as usize + length_asset_id as usize];
            let address = &memory[offset_address as usize..offset_address as usize + length_address as usize];

            match ctx.stack.get_balance(asset_id, address) {
                Ok(result) => (0, result),
                Err(error) => (error.as_i32(), 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn Transfer(
        offset_asset_id: u32,
        length_asset_id: u32,
        offset_recipient: u32,
        length_recipient: u32,
        amount: i64,
    ) -> i32 {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return RuntimeError::MemoryNotFound as i32,
            };

            let asset_id = &memory[offset_asset_id as usize..offset_asset_id as usize + length_asset_id as usize];
            let recipient = &memory[offset_recipient as usize..offset_recipient as usize + length_recipient as usize];

            match ctx.stack.transfer(asset_id, recipient, amount) {
                Ok(_) => 0,
                Err(error) => error.as_i32(),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn Issue(
        offset_name: u32,
        length_name: u32,
        offset_description: u32,
        length_description: u32,
        quantity: i64,
        decimals: i32,
        is_reissuable: i32,
    ) -> (i32, i32, i32) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
            };
            let offset_memory = ctx.heap_base() as usize;

            let name = &memory[offset_name as usize..offset_name as usize + length_name as usize];
            let description = &memory[offset_description as usize..offset_description as usize + length_description as usize];

            match ctx.stack.issue(name, description, quantity, decimals, is_reissuable != 0) {
                Ok(result) => write_memory!(ctx, memory, offset_memory, result),
                Err(error) => (error.as_i32(), 0, 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn Burn(
        offset_asset_id: u32,
        length_asset_id: u32,
        amount: i64,
    ) -> i32 {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return RuntimeError::MemoryNotFound as i32,
            };

            let asset_id = &memory[offset_asset_id as usize..offset_asset_id as usize + length_asset_id as usize];

            match ctx.stack.burn(asset_id, amount) {
                Ok(_) => 0,
                Err(error) => error.as_i32(),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn Reissue(
        offset_asset_id: u32,
        length_asset_id: u32,
        amount: i64,
        is_reissuable: i32,
    ) -> i32 {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return RuntimeError::MemoryNotFound as i32,
            };

            let asset_id = &memory[offset_asset_id as usize..offset_asset_id as usize + length_asset_id as usize];

            match ctx.stack.reissue(asset_id, amount, is_reissuable != 0) {
                Ok(_) => 0,
                Err(error) => error.as_i32(),
            }
        }
    }
}

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

            let recipient = &memory[offset_recipient as usize..offset_recipient as usize + length_recipient as usize];

            match ctx.stack.lease(recipient, amount) {
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

            let lease_id = &memory[offset_lease_id as usize..offset_lease_id as usize + length_lease_id as usize];

            match ctx.stack.cancel_lease(lease_id) {
                Ok(_) => 0,
                Err(error) => error.as_i32(),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn GetBlockTimestamp() -> (i32, i64) {
        |caller: Caller<Runtime>| {
            match caller.data().stack.get_block_timestamp() {
                Ok(result) => (0, result),
                Err(error) => (error.as_i32(), 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn GetBlockHeight() -> (i32, i64) {
        |caller: Caller<Runtime>| {
            match caller.data().stack.get_block_height() {
                Ok(result) => (0, result),
                Err(error) => (error.as_i32(), 0),
            }
        }
    }
}

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

env_runtime! {
    #[version = 0]
    pub fn GetTxPayments() -> (i32, i32) {
        |caller: Caller<Runtime>| {
            match caller.data().stack.get_tx_payments() {
                Ok(result) => (0, result),
                Err(error) => (error.as_i32(), 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn GetTxPaymentAssetId(number: i32) -> (i32, i32, i32) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
            };
            let offset_memory = ctx.heap_base() as usize;

            match ctx.stack.get_tx_payment_asset_id(number) {
                Ok(result) => write_memory!(ctx, memory, offset_memory, result),
                Err(error) => (error.as_i32(), 0, 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn GetTxPaymentAmount(number: i32) -> (i32, i64) {
        |caller: Caller<Runtime>| {
            match caller.data().stack.get_tx_payment_amount(number) {
                Ok(result) => (0, result),
                Err(error) => (error.as_i32(), 0),
            }
        }
    }
}
