use crate::{error::RuntimeError, node::Node, runtime::Runtime};
use blake2::{digest::consts::U32, Blake2b, Digest};
use sha2::Sha256;
use sha3::Keccak256;
use wasmi::Caller;

pub fn fast_hash(
    offset_bytes: u32,
    length_bytes: u32,
    mut caller: Caller<Runtime>,
) -> (i32, u32, u32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let bytes = &memory[offset_bytes as usize..offset_bytes as usize + length_bytes as usize];

    match ctx.vm.fast_hash(bytes) {
        Ok(result) => crate::env::write_memory(ctx, memory, offset_memory, result),
        Err(error) => (error.as_i32(), 0, 0),
    }
}

pub fn secure_hash(
    offset_bytes: u32,
    length_bytes: u32,
    mut caller: Caller<Runtime>,
) -> (i32, u32, u32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let bytes = &memory[offset_bytes as usize..offset_bytes as usize + length_bytes as usize];

    match ctx.vm.secure_hash(bytes) {
        Ok(result) => crate::env::write_memory(ctx, memory, offset_memory, result),
        Err(error) => (error.as_i32(), 0, 0),
    }
}

pub fn blake2b256(
    offset_bytes: u32,
    length_bytes: u32,
    mut caller: Caller<Runtime>,
) -> (i32, u32, u32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let mut hasher: Blake2b<U32> = Blake2b::new();
    hasher.update(&memory[offset_bytes as usize..offset_bytes as usize + length_bytes as usize]);

    crate::env::write_memory(ctx, memory, offset_memory, hasher.finalize().to_vec())
}

pub fn keccak256(
    offset_bytes: u32,
    length_bytes: u32,
    mut caller: Caller<Runtime>,
) -> (i32, u32, u32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let mut hasher = Keccak256::new();
    hasher.update(&memory[offset_bytes as usize..offset_bytes as usize + length_bytes as usize]);

    crate::env::write_memory(ctx, memory, offset_memory, hasher.finalize().to_vec())
}

pub fn sha256(
    offset_bytes: u32,
    length_bytes: u32,
    mut caller: Caller<Runtime>,
) -> (i32, u32, u32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let mut hasher = Sha256::new();
    hasher.update(&memory[offset_bytes as usize..offset_bytes as usize + length_bytes as usize]);

    crate::env::write_memory(ctx, memory, offset_memory, hasher.finalize().to_vec())
}

pub fn sig_verify(
    offset_message: u32,
    length_message: u32,
    offset_signature: u32,
    length_signature: u32,
    offset_public_key: u32,
    length_public_key: u32,
    mut caller: Caller<Runtime>,
) -> (i32, i32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0),
    };

    let message =
        &memory[offset_message as usize..offset_message as usize + length_message as usize];
    let signature =
        &memory[offset_signature as usize..offset_signature as usize + length_signature as usize];
    let public_key = &memory
        [offset_public_key as usize..offset_public_key as usize + length_public_key as usize];

    match ctx.vm.sig_verify(message, signature, public_key) {
        Ok(result) => (0, result as i32),
        Err(error) => (error.as_i32(), 0),
    }
}
