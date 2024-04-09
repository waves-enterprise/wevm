use crate::{error::RuntimeError, runtime::Runtime};
use std::str;
use wasmi::Caller;

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
) -> (i32, u32, u32) {
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

    crate::env::write_memory(ctx, memory, offset_memory, result)
}

pub fn contains(
    offset_bytes: u32,
    length_bytes: u32,
    offset_subbytes: u32,
    length_subbytes: u32,
    mut caller: Caller<Runtime>,
) -> (i32, i32) {
    let (memory, _) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0),
    };

    let bytes = &memory[offset_bytes as usize..offset_bytes as usize + length_bytes as usize];
    let subbytes =
        &memory[offset_subbytes as usize..offset_subbytes as usize + length_subbytes as usize];

    let result = bytes.windows(subbytes.len()).any(|item| item == subbytes);
    (0, result as i32)
}

pub fn drop(offset_bytes: u32, length_bytes: u32, n: i64) -> (i32, u32, u32) {
    match u32::try_from(n) {
        Ok(value) => (0, offset_bytes + value, length_bytes - value),
        Err(_) => (RuntimeError::ConvertingNumericTypes as i32, 0, 0),
    }
}

pub fn drop_right(offset_bytes: u32, length_bytes: u32, n: i64) -> (i32, u32, u32) {
    match u32::try_from(n) {
        Ok(value) => (0, offset_bytes, length_bytes - value),
        Err(_) => (RuntimeError::ConvertingNumericTypes as i32, 0, 0),
    }
}

pub fn index_of(
    is_last: bool,
    offset_string: u32,
    length_string: u32,
    offset_substring: u32,
    length_substring: u32,
    mut caller: Caller<Runtime>,
) -> (i32, i64) {
    let (memory, _) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0),
    };

    let string = match str::from_utf8(
        &memory[offset_string as usize..offset_string as usize + length_string as usize],
    ) {
        Ok(value) => value,
        Err(_) => return (RuntimeError::Utf8Error as i32, 0),
    };

    let substring = match str::from_utf8(
        &memory[offset_substring as usize..offset_substring as usize + length_substring as usize],
    ) {
        Ok(value) => value,
        Err(_) => return (RuntimeError::Utf8Error as i32, 0),
    };

    let result = if is_last {
        string.rfind(substring)
    } else {
        string.find(substring)
    };

    match result {
        Some(index) => (0, index as i64),
        None => (0, -1),
    }
}

pub fn take(offset_bytes: u32, _length_bytes: u32, n: i64) -> (i32, u32, u32) {
    match u32::try_from(n) {
        Ok(value) => (0, offset_bytes, value),
        Err(_) => (RuntimeError::ConvertingNumericTypes as i32, 0, 0),
    }
}

pub fn take_right(offset_bytes: u32, length_bytes: u32, n: i64) -> (i32, u32, u32) {
    match u32::try_from(n) {
        Ok(value) => (0, offset_bytes + (length_bytes - value), value),
        Err(_) => (RuntimeError::ConvertingNumericTypes as i32, 0, 0),
    }
}
