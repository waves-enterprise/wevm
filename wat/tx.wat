(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "get_tx_sender" (func $get_tx_sender (result i32 i32 i32)))
    (import "env1" "get_payments" (func $get_payments (result i32 i64)))
    (import "env1" "get_payment_asset_id" (func $get_payment_asset_id (param i64) (result i32 i32 i32)))
    (import "env1" "get_payment_amount" (func $get_payment_amount (param i64) (result i32 i64)))
    (import "env1" "tx" (func $tx (param i32 i32) (result i32 i32 i32)))

    (import "env0" "set_storage_int" (func $set_storage_int (param i32 i32 i64) (result i32)))
    (import "env0" "set_storage_binary" (func $set_storage_binary (param i32 i32 i32 i32) (result i32)))

    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "get_tx_sender") (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $get_tx_sender)

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_binary
                        (i32.const 0)
                        (i32.const 6)
                        (local.get $offset)
                        (local.get $length)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "get_payments") (result i32)
        (local $number i64) (local $error i32)
        (block $code
            (call $get_payments)

            (local.set $number)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_int
                        (i32.const 0)
                        (i32.const 6)
                        (local.get $number)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "get_payment_asset_id") (param $p0 i64) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $get_payment_asset_id
                (local.get $p0)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_binary
                        (i32.const 0)
                        (i32.const 6)
                        (local.get $offset)
                        (local.get $length)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "get_payment_amount") (param $p0 i64) (result i32)
        (local $number i64) (local $error i32)
        (block $code
            (call $get_payment_amount
                (local.get $p0)
            )

            (local.set $number)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_int
                        (i32.const 0)
                        (i32.const 6)
                        (local.get $number)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "tx") (param $p0 i32) (param $p1 i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $tx
                (local.get $p0)
                (local.get $p1)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_binary
                        (i32.const 0)
                        (i32.const 6)
                        (local.get $offset)
                        (local.get $length)
                    )
                )
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 6))

    ;; Key
    (data (i32.const 0) "result")
)
