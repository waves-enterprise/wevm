use crate::{env::Field, error::RuntimeError, node::Node, runtime::Runtime};
use wasmi::Caller;

pub fn get_block_field(field: Field, mut caller: Caller<Runtime>) -> (i32, i64) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0),
    };

    let field = match field {
        Field::String(name) => name.into_bytes(),
        Field::Binary(offset, length) => {
            memory[offset as usize..offset as usize + length as usize].to_vec()
        }
    };

    match ctx.vm.block(field.as_slice()) {
        Ok(bytes) => {
            let mut result = [0u8; 8];
            result.copy_from_slice(&bytes);
            (0, i64::from_be_bytes(result))
        }
        Err(error) => (error.as_i32(), 0),
    }
}

pub fn block(field: Field, mut caller: Caller<Runtime>) -> (i32, i32, i32) {
    let (memory, ctx) = match caller.data().memory() {
        Some(memory) => memory.data_and_store_mut(&mut caller),
        None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
    };
    let offset_memory = ctx.heap_base() as usize;

    let field = match field {
        Field::String(name) => name.into_bytes(),
        Field::Binary(offset, length) => {
            memory[offset as usize..offset as usize + length as usize].to_vec()
        }
    };

    match ctx.vm.block(field.as_slice()) {
        Ok(result) => crate::env::write_memory(ctx, memory, offset_memory, result),
        Err(error) => (error.as_i32(), 0, 0),
    }
}
