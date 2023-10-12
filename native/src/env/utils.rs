use crate::{
    env::Environment,
    env_items, env_runtime,
    runtime::{Runtime, RuntimeError},
    write_memory,
};
use base58::{FromBase58, ToBase58};
use convert_case::{Case, Casing};
use std::str;
use wasmi::{Caller, Func, Store};

env_items!(Base58, ToBase58String);

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
    pub fn ToBase58String(
        offset_bytes: u32,
        length_bytes: u32,
    ) -> (i32, i32, i32) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
            };
            let offset_memory = ctx.heap_base() as usize;

            let value = &memory[offset_bytes as usize..offset_bytes as usize + length_bytes as usize];

            let result = value.to_base58().as_bytes().to_vec();
            write_memory!(ctx, memory, offset_memory, result)
        }
    }
}
