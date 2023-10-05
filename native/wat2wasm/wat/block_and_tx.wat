(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "set_storage_int" (func $set_storage_int (param i32 i32 i64) (result i32)))
    (import "env0" "set_storage_binary" (func $set_storage_binary (param i32 i32 i32 i32) (result i32)))

    (import "env0" "get_block_timestamp" (func $get_block_timestamp (result i32 i64)))
    (import "env0" "get_block_height" (func $get_block_height (result i32 i64)))
    (import "env0" "get_tx_sender" (func $get_tx_sender (result i32 i32 i32)))
    (import "env0" "get_payments" (func $get_payments (result i32 i32)))
    (import "env0" "get_payment_asset_id" (func $get_payment_asset_id (param i32) (result i32 i32 i32)))
    (import "env0" "get_payment_amount" (func $get_payment_amount (param i32) (result i32 i64)))

    (func (export "_constructor") (result i32)
        (local $int i64) (local $offset i32) (local $length i32) (local $number i32) (local $error i32)
        (block $code
            (call $get_block_timestamp)

            (local.set $int)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_int
                        (i32.const 0)
                        (i32.const 15)
                        (local.get $int)
                    )
                )
            )

            (call $get_block_height)

            (local.set $int)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_int
                        (i32.const 15)
                        (i32.const 12)
                        (local.get $int)
                    )
                )
            )

            (call $get_tx_sender)

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_binary
                        (i32.const 27)
                        (i32.const 9)
                        (local.get $offset)
                        (local.get $length)
                    )
                )
            )

            (call $get_payments)

            (local.set $number)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_int
                        (i32.const 36)
                        (i32.const 11)
                        (i64.extend_i32_u
                            (local.get $number)
                        )
                    )
                )
            )

            (call $get_payment_asset_id
                (i32.const 1)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_binary
                        (i32.const 47)
                        (i32.const 19)
                        (local.get $offset)
                        (local.get $length)
                    )
                )
            )

            (call $get_payment_amount
                (i32.const 1)
            )

            (local.set $int)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_int
                        (i32.const 66)
                        (i32.const 17)
                        (local.get $int)
                    )
                )
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 83))

    ;; Keys
    (data (i32.const 0) "block_timestamp")
    (data (i32.const 15) "block_height")
    (data (i32.const 27) "tx_sender")
    (data (i32.const 36) "tx_payments")
    (data (i32.const 47) "tx_payment_asset_id")
    (data (i32.const 66) "tx_payment_amount")
)
