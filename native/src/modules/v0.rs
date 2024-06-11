use wevm_proc_macro::module;

#[module(env0)]
mod test {
    // Asset
    fn get_balance(
        offset_asset_id: *const u8,
        length_asset_id: usize,
        offset_address: *const u8,
        length_address: usize,
    ) -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::asset::get_balance(
                offset_asset_id,
                length_asset_id,
                offset_address,
                length_address,
                0,
                1,
                caller,
            )
        }
    }

    fn transfer(
        offset_asset_id: *const u8,
        length_asset_id: usize,
        offset_recipient: *const u8,
        length_recipient: usize,
        amount: i64,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::asset::transfer(
                offset_asset_id,
                length_asset_id,
                offset_recipient,
                length_recipient,
                0,
                1,
                amount,
                caller,
            )
        }
    }

    fn issue(
        offset_name: *const u8,
        length_name: usize,
        offset_description: *const u8,
        length_description: usize,
        quantity: i64,
        decimals: i32,
        is_reissuable: bool,
    ) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| {
            env::asset::issue(
                offset_name,
                length_name,
                offset_description,
                length_description,
                quantity,
                decimals as i64,
                is_reissuable,
                caller,
            )
        }
    }

    fn burn(offset_asset_id: *const u8, length_asset_id: usize, amount: i64) -> i32 {
        |caller: Caller<Runtime>| env::asset::burn(offset_asset_id, length_asset_id, amount, caller)
    }

    fn reissue(
        offset_asset_id: *const u8,
        length_asset_id: usize,
        amount: i64,
        is_reissuable: bool,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::asset::reissue(
                offset_asset_id,
                length_asset_id,
                amount,
                is_reissuable,
                caller,
            )
        }
    }

    // Block
    fn get_block_timestamp() -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::block::get_block_field(env::Field::String("timestamp".to_string()), caller)
        }
    }

    fn get_block_height() -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::block::get_block_field(env::Field::String("height".to_string()), caller)
        }
    }

    // Call contract
    fn call_arg_int(value: i64) {
        |caller: Caller<Runtime>| env::call_contract::call_arg_int(value, caller)
    }

    fn call_arg_bool(value: bool) {
        |caller: Caller<Runtime>| env::call_contract::call_arg_bool(value, caller)
    }

    fn call_arg_binary(offset_value: *const u8, length_value: usize) -> i32 {
        |caller: Caller<Runtime>| {
            env::call_contract::call_arg_binary(offset_value, length_value, caller)
        }
    }

    fn call_arg_string(offset_value: *const u8, length_value: usize) -> i32 {
        |caller: Caller<Runtime>| {
            env::call_contract::call_arg_string(offset_value, length_value, caller)
        }
    }

    fn call_payment(offset_asset_id: *const u8, length_asset_id: usize, amount: i64) -> i32 {
        |caller: Caller<Runtime>| {
            env::call_contract::call_payment(offset_asset_id, length_asset_id, amount, caller)
        }
    }

    fn call_contract(
        offset_contract_id: *const u8,
        length_contract_id: usize,
        offset_func_name: *const u8,
        length_func_name: usize,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::call_contract::call_contract(
                offset_contract_id,
                length_contract_id,
                offset_func_name,
                length_func_name,
                None,
                None,
                caller,
            )
        }
    }

    fn call_contract_params(
        offset_contract_id: *const u8,
        length_contract_id: usize,
        offset_func_name: *const u8,
        length_func_name: usize,
        offset_params: *const u8,
        length_params: usize,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::call_contract::call_contract(
                offset_contract_id,
                length_contract_id,
                offset_func_name,
                length_func_name,
                Some(offset_params),
                Some(length_params),
                caller,
            )
        }
    }

    // Converts
    fn parse_int(offset: *const u8, length: usize) -> (i32, i64) {
        |caller: Caller<Runtime>| env::converts::parse_int(offset, length, caller)
    }

    fn parse_bool(offset_string: *const u8, length_string: usize) -> (i32, bool) {
        |caller: Caller<Runtime>| env::converts::parse_bool(offset_string, length_string, caller)
    }

    fn to_bytes(value: i64) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::converts::to_bytes(value, caller)
    }

    fn to_int(offset: *const u8, length: usize) -> (i32, i64) {
        |caller: Caller<Runtime>| env::converts::to_int(offset, length, caller)
    }

    fn to_string_bool(value: bool) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::converts::to_string(value != 0, caller)
    }

    fn to_string_int(value: i64) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::converts::to_string(value, caller)
    }

    // Crypto
    fn fast_hash(offset_bytes: *const u8, length_bytes: usize) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::crypto::fast_hash(offset_bytes, length_bytes, caller)
    }

    fn secure_hash(offset_bytes: *const u8, length_bytes: usize) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::crypto::secure_hash(offset_bytes, length_bytes, caller)
    }

    fn blake2b256(offset_bytes: *const u8, length_bytes: usize) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::crypto::blake2b256(offset_bytes, length_bytes, caller)
    }

    fn keccak256(offset_bytes: *const u8, length_bytes: usize) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::crypto::keccak256(offset_bytes, length_bytes, caller)
    }

    fn sha256(offset_bytes: *const u8, length_bytes: usize) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::crypto::sha256(offset_bytes, length_bytes, caller)
    }

    fn sig_verify(
        offset_message: *const u8,
        length_message: usize,
        offset_signature: *const u8,
        length_signature: usize,
        offset_public_key: *const u8,
        length_public_key: usize,
    ) -> (i32, bool) {
        |caller: Caller<Runtime>| {
            env::crypto::sig_verify(
                offset_message,
                length_message,
                offset_signature,
                length_signature,
                offset_public_key,
                length_public_key,
                caller,
            )
        }
    }

    // Lease
    fn lease_address(
        offset_address: *const u8,
        length_address: usize,
        amount: i64,
    ) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| {
            env::lease::lease(offset_address, length_address, 1, amount, caller)
        }
    }

    fn lease_alias(
        offset_alias: *const u8,
        length_alias: usize,
        amount: i64,
    ) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::lease::lease(offset_alias, length_alias, 2, amount, caller)
    }

    fn cancel_lease(offset_lease_id: *const u8, length_lease_id: usize) -> i32 {
        |caller: Caller<Runtime>| env::lease::cancel_lease(offset_lease_id, length_lease_id, caller)
    }

    // Memory
    fn binary_equals(
        offset_left: *const u8,
        length_left: usize,
        offset_right: *const u8,
        length_right: usize,
    ) -> (i32, bool) {
        |caller: Caller<Runtime>| {
            env::memory::binary_equals(offset_left, length_left, offset_right, length_right, caller)
        }
    }

    fn string_equals(
        offset_left: *const u8,
        length_left: usize,
        offset_right: *const u8,
        length_right: usize,
    ) -> (i32, bool) {
        |caller: Caller<Runtime>| {
            env::memory::string_equals(offset_left, length_left, offset_right, length_right, caller)
        }
    }

    fn join(
        offset_left: *const u8,
        length_left: usize,
        offset_right: *const u8,
        length_right: usize,
    ) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| {
            env::memory::join(offset_left, length_left, offset_right, length_right, caller)
        }
    }

    fn contains(
        offset_bytes: *const u8,
        length_bytes: usize,
        offset_subbytes: *const u8,
        length_subbytes: usize,
    ) -> (i32, bool) {
        |caller: Caller<Runtime>| {
            env::memory::contains(
                offset_bytes,
                length_bytes,
                offset_subbytes,
                length_subbytes,
                caller,
            )
        }
    }

    fn drop(offset_bytes: *const u8, length_bytes: usize, n: i64) -> (i32, *const u8, usize) {
        |_caller: Caller<Runtime>| env::memory::drop(offset_bytes, length_bytes, n)
    }

    fn drop_right(offset_bytes: *const u8, length_bytes: usize, n: i64) -> (i32, *const u8, usize) {
        |_caller: Caller<Runtime>| env::memory::drop_right(offset_bytes, length_bytes, n)
    }

    fn index_of(
        offset_string: *const u8,
        length_string: usize,
        offset_substring: *const u8,
        length_substring: usize,
    ) -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::memory::index_of(
                false,
                offset_string,
                length_string,
                offset_substring,
                length_substring,
                caller,
            )
        }
    }

    fn last_index_of(
        offset_string: *const u8,
        length_string: usize,
        offset_substring: *const u8,
        length_substring: usize,
    ) -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::memory::index_of(
                true,
                offset_string,
                length_string,
                offset_substring,
                length_substring,
                caller,
            )
        }
    }

    fn take(offset_bytes: *const u8, length_bytes: usize, n: i64) -> (i32, *const u8, usize) {
        |_caller: Caller<Runtime>| env::memory::take(offset_bytes, length_bytes, n)
    }

    fn take_right(offset_bytes: *const u8, length_bytes: usize, n: i64) -> (i32, *const u8, usize) {
        |_caller: Caller<Runtime>| env::memory::take_right(offset_bytes, length_bytes, n)
    }

    // Storage
    fn contains_key(
        offset_address: *const u8,
        length_address: usize,
        offset_key: *const u8,
        length_key: usize,
    ) -> (i32, bool) {
        |caller: Caller<Runtime>| {
            env::storage::contains_key(
                offset_address,
                length_address,
                offset_key,
                length_key,
                caller,
            )
        }
    }

    fn get_storage_int(
        offset_address: *const u8,
        length_address: usize,
        offset_key: *const u8,
        length_key: usize,
    ) -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::storage::get_storage_int(
                offset_address,
                length_address,
                offset_key,
                length_key,
                caller,
            )
        }
    }

    fn get_storage_bool(
        offset_address: *const u8,
        length_address: usize,
        offset_key: *const u8,
        length_key: usize,
    ) -> (i32, bool) {
        |caller: Caller<Runtime>| {
            env::storage::get_storage_bool(
                offset_address,
                length_address,
                offset_key,
                length_key,
                caller,
            )
        }
    }

    fn get_storage_binary(
        offset_address: *const u8,
        length_address: usize,
        offset_key: *const u8,
        length_key: usize,
    ) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| {
            env::storage::get_storage_binary(
                offset_address,
                length_address,
                offset_key,
                length_key,
                caller,
            )
        }
    }

    fn get_storage_string(
        offset_address: *const u8,
        length_address: usize,
        offset_key: *const u8,
        length_key: usize,
    ) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| {
            env::storage::get_storage_string(
                offset_address,
                length_address,
                offset_key,
                length_key,
                caller,
            )
        }
    }

    fn set_storage_int(offset_key: *const u8, length_key: usize, value: i64) -> i32 {
        |caller: Caller<Runtime>| {
            env::storage::set_storage_int(offset_key, length_key, value, caller)
        }
    }

    fn set_storage_bool(offset_key: *const u8, length_key: usize, value: bool) -> i32 {
        |caller: Caller<Runtime>| {
            env::storage::set_storage_bool(offset_key, length_key, value, caller)
        }
    }

    fn set_storage_binary(
        offset_key: *const u8,
        length_key: usize,
        offset_value: *const u8,
        length_value: usize,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::storage::set_storage_binary(
                offset_key,
                length_key,
                offset_value,
                length_value,
                caller,
            )
        }
    }

    fn set_storage_string(
        offset_key: *const u8,
        length_key: usize,
        offset_value: *const u8,
        length_value: usize,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::storage::set_storage_string(
                offset_key,
                length_key,
                offset_value,
                length_value,
                caller,
            )
        }
    }

    // Tx
    fn get_tx_sender() -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::tx::tx(env::Field::String("sender".to_string()), caller)
    }

    fn get_payments() -> (i32, i32) {
        |caller: Caller<Runtime>| {
            let (err, num) = env::tx::get_payments(caller);
            match i32::try_from(num) {
                Ok(value) => (err, value),
                Err(_) => (
                    Error::Runtime(RuntimeError::ConvertingNumericTypes).as_i32(),
                    0,
                ),
            }
        }
    }

    fn get_payment_asset_id(number: i32) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::tx::get_payment_asset_id(number as i64, caller)
    }

    fn get_payment_amount(number: i32) -> (i32, i64) {
        |caller: Caller<Runtime>| env::tx::get_payment_amount(number as i64, caller)
    }

    // Utils
    fn base_58(offset_bytes: *const u8, length_bytes: usize) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::utils::base58(offset_bytes, length_bytes, caller)
    }

    fn to_base_58_string(offset_bytes: *const u8, length_bytes: usize) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::utils::to_base58_string(offset_bytes, length_bytes, caller)
    }

    fn to_le_bytes(offset_bytes: *const u8, length_bytes: usize) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::utils::to_le_bytes(offset_bytes, length_bytes, caller)
    }

    fn caller() -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::utils::caller(caller)
    }

    fn require(offset_message: *const u8, length_message: usize) -> i32 {
        |caller: Caller<Runtime>| env::utils::require(offset_message, length_message, caller)
    }
}
