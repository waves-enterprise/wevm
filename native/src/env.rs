mod asset;
mod block;
mod call_contract;
mod lease;
mod payments;
mod storage;
mod tx;
mod utils;

use crate::runtime::Runtime;
use dyn_clone::DynClone;
use wasmi::{Func, Store};

pub trait Environment: DynClone {
    fn module(&self) -> String;
    fn name(&self) -> String;
    fn func(&self, store: &mut Store<Runtime>) -> Func;
}

dyn_clone::clone_trait_object!(Environment);

pub fn envs() -> Vec<Box<dyn Environment>> {
    let mut result: Vec<Box<dyn Environment>> = vec![];

    result.extend(asset::to_vec());
    result.extend(block::to_vec());
    result.extend(call_contract::to_vec());
    result.extend(lease::to_vec());
    result.extend(payments::to_vec());
    result.extend(storage::to_vec());
    result.extend(tx::to_vec());
    result.extend(utils::to_vec());

    result
}
