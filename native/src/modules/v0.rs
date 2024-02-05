use crate::{
    env,
    error::{Error, RuntimeError},
    module,
    modules::Module,
    runtime::Runtime,
};
use wasmi::{Caller, Func, Store};

module! {
    // Asset
    #[version = 0]
    pub fn get_balance(
        offset_asset_id: u32,
        length_asset_id: u32,
        offset_address: u32,
        length_address: u32,
    ) -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::asset::get_balance(offset_asset_id,
                                    length_asset_id,
                                    offset_address,
                                    length_address,
                                    caller)
        }
    }

    #[version = 0]
    pub fn transfer(
        offset_asset_id: u32,
        length_asset_id: u32,
        offset_recipient: u32,
        length_recipient: u32,
        amount: i64,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::asset::transfer(offset_asset_id,
                                 length_asset_id,
                                 offset_recipient,
                                 length_recipient,
                                 amount,
                                 caller)
        }
    }

    #[version = 0]
    pub fn issue(
        offset_name: u32,
        length_name: u32,
        offset_description: u32,
        length_description: u32,
        quantity: i64,
        decimals: i32,
        is_reissuable: i32,
    ) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::asset::issue(offset_name,
                              length_name,
                              offset_description,
                              length_description,
                              quantity,
                              decimals as i64,
                              is_reissuable,
                              caller)
        }
    }

    #[version = 0]
    pub fn burn(
        offset_asset_id: u32,
        length_asset_id: u32,
        amount: i64,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::asset::burn(offset_asset_id,
                             length_asset_id,
                             amount,
                             caller)
        }
    }

    #[version = 0]
    pub fn reissue(
        offset_asset_id: u32,
        length_asset_id: u32,
        amount: i64,
        is_reissuable: i32,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::asset::reissue(offset_asset_id,
                                length_asset_id,
                                amount,
                                is_reissuable,
                                caller)
        }
    }

    // Block
    #[version = 0]
    pub fn get_block_timestamp() -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::block::get_block_timestamp(caller)
        }
    }

    #[version = 0]
    pub fn get_block_height() -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::block::get_block_height(caller)
        }
    }

    // Call contract
    #[version = 0]
    pub fn call_arg_int(value: i64) {
        |caller: Caller<Runtime>| {
            env::call_contract::call_arg_int(value, caller)
        }
    }

    #[version = 0]
    pub fn call_arg_bool(value: i32) {
        |caller: Caller<Runtime>| {
            env::call_contract::call_arg_bool(value, caller)
        }
    }

    #[version = 0]
    pub fn call_arg_binary(offset_value: u32, length_value: u32) -> i32 {
        |caller: Caller<Runtime>| {
            env::call_contract::call_arg_binary(offset_value,
                                                length_value,
                                                caller)
        }
    }

    #[version = 0]
    pub fn call_arg_string(offset_value: u32, length_value: u32) -> i32 {
        |caller: Caller<Runtime>| {
            env::call_contract::call_arg_string(offset_value,
                                                length_value,
                                                caller)
        }
    }

    #[version = 0]
    pub fn call_payment(offset_asset_id: u32, length_asset_id: u32, amount: i64) -> i32 {
        |caller: Caller<Runtime>| {
            env::call_contract::call_payment(offset_asset_id,
                                             length_asset_id,
                                             amount,
                                             caller)
        }
    }

    #[version = 0]
    pub fn call_contract(
        offset_contract_id: u32,
        length_contract_id: u32,
        offset_func_name: u32,
        length_func_name: u32,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::call_contract::call_contract(offset_contract_id,
                                              length_contract_id,
                                              offset_func_name,
                                              length_func_name,
                                              caller)
        }
    }

    // Lease
    #[version = 0]
    pub fn lease_address(
        offset_address: u32,
        length_address: u32,
        amount: i64,
    ) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::lease::lease(1,
                              offset_address,
                              length_address,
                              amount,
                              caller)
        }
    }

    #[version = 0]
    pub fn lease_alias(
        offset_alias: u32,
        length_alias: u32,
        amount: i64,
    ) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::lease::lease(2,
                              offset_alias,
                              length_alias,
                              amount,
                              caller)
        }
    }

    #[version = 0]
    pub fn cancel_lease(
        offset_lease_id: u32,
        length_lease_id: u32,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::lease::cancel_lease(offset_lease_id,
                                     length_lease_id,
                                     caller)
        }
    }

    // Payments
    #[version = 0]
    pub fn get_payments() -> (i32, i32) {
        |caller: Caller<Runtime>| {
            let (err, num) = env::payments::get_payments(caller);
            match i32::try_from(num) {
                Ok(value) => (err, value),
                Err(_) => (Error::Runtime(RuntimeError::ConvertingNumericTypes).as_i32(), 0),
            }
        }
    }

    #[version = 0]
    pub fn get_payment_asset_id(number: i32) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::payments::get_payment_asset_id(number as i64, caller)
        }
    }

    #[version = 0]
    pub fn get_payment_amount(number: i32) -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::payments::get_payment_amount(number as i64, caller)
        }
    }

    // Storage
    #[version = 0]
    pub fn get_storage_int(
        offset_address: u32,
        length_address: u32,
        offset_key: u32,
        length_key: u32,
    ) -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::storage::get_storage_int(offset_address,
                                          length_address,
                                          offset_key,
                                          length_key,
                                          caller)
        }
    }

    #[version = 0]
    pub fn get_storage_bool(
        offset_address: u32,
        length_address: u32,
        offset_key: u32,
        length_key: u32,
    ) -> (i32, i32) {
        |caller: Caller<Runtime>| {
            env::storage::get_storage_bool(offset_address,
                                           length_address,
                                           offset_key,
                                           length_key,
                                           caller)
        }
    }

    #[version = 0]
    pub fn get_storage_binary(
        offset_address: u32,
        length_address: u32,
        offset_key: u32,
        length_key: u32,
    ) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::storage::get_storage_binary(offset_address,
                                             length_address,
                                             offset_key,
                                             length_key,
                                             caller)
        }
    }

    #[version = 0]
    pub fn get_storage_string(
        offset_address: u32,
        length_address: u32,
        offset_key: u32,
        length_key: u32,
    ) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::storage::get_storage_string(offset_address,
                                             length_address,
                                             offset_key,
                                             length_key,
                                             caller)
        }
    }

    #[version = 0]
    pub fn set_storage_int(
        offset_key: u32,
        length_key: u32,
        value: i64,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::storage::set_storage_int(offset_key,
                                          length_key,
                                          value,
                                          caller)
        }
    }

    #[version = 0]
    pub fn set_storage_bool(
        offset_key: u32,
        length_key: u32,
        value: i32,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::storage::set_storage_bool(offset_key,
                                           length_key,
                                           value,
                                           caller)
        }
    }

    #[version = 0]
    pub fn set_storage_binary(
        offset_key: u32,
        length_key: u32,
        offset_value: u32,
        length_value: u32,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::storage::set_storage_binary(offset_key,
                                             length_key,
                                             offset_value,
                                             length_value,
                                             caller)
        }
    }

    #[version = 0]
    pub fn set_storage_string(
        offset_key: u32,
        length_key: u32,
        offset_value: u32,
        length_value: u32,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::storage::set_storage_string(offset_key,
                                             length_key,
                                             offset_value,
                                             length_value,
                                             caller)
        }
    }

    // Tx
    #[version = 0]
    pub fn get_tx_sender() -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::tx::get_tx_sender(caller)
        }
    }

    // Utils
    #[version = 0]
    pub fn base_58(
        offset_bytes: u32,
        length_bytes: u32,
    ) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::utils::base58(offset_bytes,
                               length_bytes,
                               caller)
        }
    }

    #[version = 0]
    pub fn to_base_58_string(
        offset_bytes: u32,
        length_bytes: u32,
    ) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::utils::to_base58_string(offset_bytes,
                                         length_bytes,
                                         caller)
        }
    }

    #[version = 0]
    pub fn binary_equals(
        offset_left: u32,
        length_left: u32,
        offset_right: u32,
        length_right: u32,
    ) -> (i32, i32) {
        |caller: Caller<Runtime>| {
            env::utils::binary_equals(offset_left,
                                      length_left,
                                      offset_right,
                                      length_right,
                                      caller)
        }
    }

    #[version = 0]
    pub fn string_equals(
        offset_left: u32,
        length_left: u32,
        offset_right: u32,
        length_right: u32,
    ) -> (i32, i32) {
        |caller: Caller<Runtime>| {
            env::utils::string_equals(offset_left,
                                      length_left,
                                      offset_right,
                                      length_right,
                                      caller)
        }
    }

    #[version = 0]
    pub fn join(
        offset_left: u32,
        length_left: u32,
        offset_right: u32,
        length_right: u32,
    ) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::utils::join(offset_left,
                             length_left,
                             offset_right,
                             length_right,
                             caller)
        }
    }
}
