use wevm_proc_macro::module;

#[module(env1)]
mod test {
    // Asset
    fn get_balance(
        offset_asset_id: u32,
        length_asset_id: u32,
        offset_holder: u32,
        length_holder: u32,
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
        offset_asset_id: u32,
        length_asset_id: u32,
        offset_recipient: u32,
        length_recipient: u32,
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
        offset_name: u32,
        length_name: u32,
        offset_description: u32,
        length_description: u32,
        quantity: i64,
        decimals: i64,
        is_reissuable: i32,
    ) -> (i32, i32, i32) {
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
    fn block(offset_field: u32, length_field: u32) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::block::block(env::Field::Binary(offset_field, length_field), caller)
        }
    }

    // Tx
    fn get_payments() -> (i32, i64) {
        |caller: Caller<Runtime>| env::tx::get_payments(caller)
    }

    fn get_payment_asset_id(number: i64) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| env::tx::get_payment_asset_id(number, caller)
    }

    fn get_payment_amount(number: i64) -> (i32, i64) {
        |caller: Caller<Runtime>| env::tx::get_payment_amount(number, caller)
    }

    fn tx(offset_field: u32, length_field: u32) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::tx::tx(env::Field::Binary(offset_field, length_field), caller)
        }
    }
}
