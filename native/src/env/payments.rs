use crate::{
    env::Environment, env_items, env_runtime, error::RuntimeError, node::Node, runtime::Runtime,
    write_memory,
};
use convert_case::{Case, Casing};
use wasmi::{Caller, Func, Store};

env_items!(GetPayments, GetPaymentAssetId, GetPaymentAmount);

env_runtime! {
    #[version = 0]
    pub fn GetPayments() -> (i32, i64) {
        |caller: Caller<Runtime>| {
            let payment_id = caller.data().vm.top_frame().payment_id();

            match caller.data().vm.get_tx_payments(payment_id.as_slice()) {
                Ok(result) => (0, result),
                Err(error) => (error.as_i32(), 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn GetPaymentAssetId(number: i64) -> (i32, i32, i32) {
        |mut caller: Caller<Runtime>| {
            let (memory, ctx) = match caller.data().memory() {
                Some(memory) => memory.data_and_store_mut(&mut caller),
                None => return (RuntimeError::MemoryNotFound as i32, 0, 0),
            };
            let offset_memory = ctx.heap_base() as usize;

            let payment_id = ctx.vm.top_frame().payment_id();

            match ctx.vm.get_tx_payment_asset_id(payment_id.as_slice(), number) {
                Ok(result) => write_memory!(ctx, memory, offset_memory, result),
                Err(error) => (error.as_i32(), 0, 0),
            }
        }
    }
}

env_runtime! {
    #[version = 0]
    pub fn GetPaymentAmount(number: i64) -> (i32, i64) {
        |caller: Caller<Runtime>| {
            let payment_id = caller.data().vm.top_frame().payment_id();

            match caller.data().vm.get_tx_payment_amount(payment_id.as_slice(), number) {
                Ok(result) => (0, result),
                Err(error) => (error.as_i32(), 0),
            }
        }
    }
}
