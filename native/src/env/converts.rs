use crate::{error::RuntimeError, runtime::Runtime};
use std::{fmt::Display, str};
use wasmi::Caller;

pub fn parse_int(offset: u32, length: u32, mut caller: Caller<Runtime>) -> (i32, i64) {
    let (memory, _) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound.as_i32(), 0),
    };

    let string = match str::from_utf8(&memory[offset as usize..offset as usize + length as usize]) {
        Ok(value) => value,
        Err(_) => return (RuntimeError::Utf8Error.as_i32(), 0),
    };

    match string.parse::<i64>() {
        Ok(value) => (0, value),
        Err(_) => (RuntimeError::ParseError.as_i32(), 0),
    }
}

pub fn parse_bool(offset: u32, length: u32, mut caller: Caller<Runtime>) -> (i32, i32) {
    let (memory, _) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound.as_i32(), 0),
    };

    let string = match str::from_utf8(&memory[offset as usize..offset as usize + length as usize]) {
        Ok(value) => value,
        Err(_) => return (RuntimeError::Utf8Error.as_i32(), 0),
    };

    match string.parse::<bool>() {
        Ok(value) => (0, value as i32),
        Err(_) => (RuntimeError::ParseError.as_i32(), 0),
    }
}

pub fn to_bytes(value: i64, mut caller: Caller<Runtime>) -> (i32, u32, u32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound.as_i32(), 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let result = value.to_be_bytes().to_vec();
    crate::env::write_memory(ctx, memory, offset_memory, result)
}

pub fn to_int(offset: u32, length: u32, mut caller: Caller<Runtime>) -> (i32, i64) {
    let (memory, _) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound.as_i32(), 0),
    };

    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&memory[offset as usize..offset as usize + length as usize]);

    (0, i64::from_be_bytes(bytes))
}

pub fn to_string<T: Display>(value: T, mut caller: Caller<Runtime>) -> (i32, u32, u32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound.as_i32(), 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let result = value.to_string().into_bytes();
    crate::env::write_memory(ctx, memory, offset_memory, result)
}
