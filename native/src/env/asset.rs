use crate::{
    env::Environment, env_items, env_runtime, error::RuntimeError, node::Node, runtime::Runtime,
    write_memory,
};
use convert_case::{Case, Casing};
use wasmi::{Caller, Func, Store};

env_items!(GetBalance, Transfer, Issue, Burn, Reissue);

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
            let address = if length_address != 0 {
                memory[offset_address as usize..offset_address as usize + length_address as usize].to_vec()
            } else {
                ctx.vm.top_frame().contract_id()
            };

            match ctx.vm.get_balance(asset_id, address.as_slice()) {
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

            let contract_id = ctx.vm.top_frame().contract_id();
            let asset_id = &memory[offset_asset_id as usize..offset_asset_id as usize + length_asset_id as usize];
            let recipient = &memory[offset_recipient as usize..offset_recipient as usize + length_recipient as usize];

            match ctx.vm.transfer(contract_id.as_slice(), asset_id, recipient, amount) {
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
        decimals: i64,
        is_reissuable: i32,
    ) -> (i32, i32, i32) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
            };
            let offset_memory = ctx.heap_base() as usize;

            let contract_id = ctx.vm.top_frame().contract_id();
            let name = &memory[offset_name as usize..offset_name as usize + length_name as usize];
            let description = &memory[offset_description as usize..offset_description as usize + length_description as usize];

            match ctx.vm.issue(contract_id.as_slice(), name, description, quantity, decimals, is_reissuable != 0) {
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

            let contract_id = ctx.vm.top_frame().contract_id();
            let asset_id = &memory[offset_asset_id as usize..offset_asset_id as usize + length_asset_id as usize];

            match ctx.vm.burn(contract_id.as_slice(), asset_id, amount) {
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

            let contract_id = ctx.vm.top_frame().contract_id();
            let asset_id = &memory[offset_asset_id as usize..offset_asset_id as usize + length_asset_id as usize];

            match ctx.vm.reissue(contract_id.as_slice(), asset_id, amount, is_reissuable != 0) {
                Ok(_) => 0,
                Err(error) => error.as_i32(),
            }
        }
    }
}
