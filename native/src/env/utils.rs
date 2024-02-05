use crate::{error::RuntimeError, runtime::Runtime, write_memory};
use base58::{FromBase58, ToBase58};
use std::str;
use wasmi::Caller;

pub fn base58(
    offset_bytes: u32,
    length_bytes: u32,
    mut caller: Caller<Runtime>,
) -> (i32, i32, i32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let value = match str::from_utf8(
        &memory[offset_bytes as usize..offset_bytes as usize + length_bytes as usize],
    ) {
        Ok(string) => string,
        Err(_) => return (RuntimeError::Utf8Error as i32, 0, 0),
    };

    match value.from_base58() {
        Ok(result) => write_memory!(ctx, memory, offset_memory, result),
        Err(_) => (RuntimeError::Base58Error as i32, 0, 0),
    }
}

pub fn to_base58_string(
    offset_bytes: u32,
    length_bytes: u32,
    mut caller: Caller<Runtime>,
) -> (i32, i32, i32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let value = &memory[offset_bytes as usize..offset_bytes as usize + length_bytes as usize];

    let result = value.to_base58().as_bytes().to_vec();
    write_memory!(ctx, memory, offset_memory, result)
}

pub fn binary_equals(
    offset_left: u32,
    length_left: u32,
    offset_right: u32,
    length_right: u32,
    mut caller: Caller<Runtime>,
) -> (i32, i32) {
    let (memory, _) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0),
    };

    let left = &memory[offset_left as usize..offset_left as usize + length_left as usize];
    let right = &memory[offset_right as usize..offset_right as usize + length_right as usize];

    (0, (left == right) as i32)
}

pub fn string_equals(
    offset_left: u32,
    length_left: u32,
    offset_right: u32,
    length_right: u32,
    mut caller: Caller<Runtime>,
) -> (i32, i32) {
    let (memory, _) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0),
    };

    let left = match str::from_utf8(
        &memory[offset_left as usize..offset_left as usize + length_left as usize],
    ) {
        Ok(string) => string,
        Err(_) => return (RuntimeError::Utf8Error as i32, 0),
    };

    let right = match str::from_utf8(
        &memory[offset_right as usize..offset_right as usize + length_right as usize],
    ) {
        Ok(string) => string,
        Err(_) => return (RuntimeError::Utf8Error as i32, 0),
    };

    (0, (left == right) as i32)
}

pub fn join(
    offset_left: u32,
    length_left: u32,
    offset_right: u32,
    length_right: u32,
    mut caller: Caller<Runtime>,
) -> (i32, i32, i32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let left = &memory[offset_left as usize..offset_left as usize + length_left as usize];
    let right = &memory[offset_right as usize..offset_right as usize + length_right as usize];

    let mut result = vec![];
    result.extend_from_slice(left);
    result.extend_from_slice(right);

    write_memory!(ctx, memory, offset_memory, result)
}
