pub mod asset;
pub mod block;
pub mod call_contract;
pub mod converts;
pub mod crypto;
pub mod lease;
pub mod storage;
pub mod tx;
pub mod utils;

use crate::{
    error::Result,
    node::Node,
    runtime::{
        asset_holder::{AddressVersion, AssetHolder, Type},
        Runtime,
    },
};

pub enum Field {
    String(String),
    Binary(u32, u32),
}

pub(in crate::env) fn get_asset_holder(
    ctx: &mut Runtime,
    type_: u32,
    version: u32,
    bytes: Vec<u8>,
) -> Result<Vec<u8>> {
    let type_ = Type::try_from(type_)?;
    let version = AddressVersion::try_from(version)?;
    let chain_id = ctx.vm.get_chain_id()? as u8;
    Ok(AssetHolder::from_bytes(type_, version, chain_id, bytes).as_bytes())
}

/// Wrapper over writing to WASM linear memory.
/// Functions using this wrapper return (i32, i32, i32):
/// * First value - error code
/// * Second value - memory offset
/// * Third value - length of data in memory
pub(in crate::env) fn write_memory(
    ctx: &mut Runtime,
    memory: &mut [u8],
    offset_memory: usize,
    result: Vec<u8>,
) -> (i32, i32, i32) {
    let length = result.len();
    memory[offset_memory..offset_memory + length].copy_from_slice(result.as_slice());
    ctx.set_heap_base((offset_memory + length) as i32);
    (0, offset_memory as i32, length as i32)
}
