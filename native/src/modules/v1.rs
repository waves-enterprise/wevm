use wevm_proc_macro::module;

#[module(env1)]
mod test {
    // Asset
    fn get_balance(
        offset_asset_id: *const u8,
        length_asset_id: usize,
        offset_holder: *const u8,
        length_holder: usize,
        type_: u32,
        version: u32,
    ) -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::asset::get_balance(
                offset_asset_id,
                length_asset_id,
                offset_holder,
                length_holder,
                type_,
                version,
                caller,
            )
        }
    }

    fn transfer(
        offset_asset_id: *const u8,
        length_asset_id: usize,
        offset_recipient: *const u8,
        length_recipient: usize,
        type_: u32,
        version: u32,
        amount: i64,
    ) -> i32 {
        |caller: Caller<Runtime>| {
            env::asset::transfer(
                offset_asset_id,
                length_asset_id,
                offset_recipient,
                length_recipient,
                type_,
                version,
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
        decimals: i64,
        is_reissuable: bool,
    ) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| {
            env::asset::issue(
                offset_name,
                length_name,
                offset_description,
                length_description,
                quantity,
                decimals,
                is_reissuable,
                caller,
            )
        }
    }

    // Block
    fn block(offset_field: *const u8, length_field: usize) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| {
            env::block::block(env::Field::Binary(offset_field, length_field), caller)
        }
    }

    // Tx
    fn get_payments() -> (i32, i64) {
        |caller: Caller<Runtime>| env::tx::get_payments(caller)
    }

    fn get_payment_asset_id(number: i64) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| env::tx::get_payment_asset_id(number, caller)
    }

    fn get_payment_amount(number: i64) -> (i32, i64) {
        |caller: Caller<Runtime>| env::tx::get_payment_amount(number, caller)
    }

    fn tx(offset_field: *const u8, length_field: usize) -> (i32, *const u8, usize) {
        |caller: Caller<Runtime>| {
            env::tx::tx(env::Field::Binary(offset_field, length_field), caller)
        }
    }
}
