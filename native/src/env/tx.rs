use crate::{env::Field, error::RuntimeError, node::Node, runtime::Runtime};
use wasmi::Caller;

pub fn tx(field: Field, mut caller: Caller<Runtime>) -> (i32, i32, i32) {
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

    match ctx.vm.tx(field.as_slice()) {
        Ok(result) => crate::env::write_memory(ctx, memory, offset_memory, result),
        Err(error) => (error.as_i32(), 0, 0),
    }
}
