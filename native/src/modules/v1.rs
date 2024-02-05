use crate::{env, module, modules::Module, runtime::Runtime};
use wasmi::{Caller, Func, Store};

module! {
    // Asset
    #[version = 1]
    pub fn issue(
        offset_name: u32,
        length_name: u32,
        offset_description: u32,
        length_description: u32,
        quantity: i64,
        decimals: i64,
        is_reissuable: i32,
    ) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::asset::issue(offset_name,
                              length_name,
                              offset_description,
                              length_description,
                              quantity,
                              decimals,
                              is_reissuable,
                              caller)
        }
    }

    // Payments
    #[version = 1]
    pub fn get_payments() -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::payments::get_payments(caller)
        }
    }

    #[version = 1]
    pub fn get_payment_asset_id(number: i64) -> (i32, i32, i32) {
        |caller: Caller<Runtime>| {
            env::payments::get_payment_asset_id(number, caller)
        }
    }

    #[version = 1]
    pub fn get_payment_amount(number: i64) -> (i32, i64) {
        |caller: Caller<Runtime>| {
            env::payments::get_payment_amount(number, caller)
        }
    }
}
