(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "get_tx_sender" (func $env0_get_tx_sender (result i32 i32 i32)))
    (import "env0" "get_payments" (func $env0_get_payments (result i32 i32)))
    (import "env0" "get_payment_asset_id" (func $env0_get_payment_asset_id (param i32) (result i32 i32 i32)))
    (import "env0" "get_payment_amount" (func $env0_get_payment_amount (param i32) (result i32 i64)))

    (import "env1" "get_payments" (func $env1_get_payments (result i32 i64)))
    (import "env1" "get_payment_asset_id" (func $env1_get_payment_asset_id (param i64) (result i32 i32 i32)))
    (import "env1" "get_payment_amount" (func $env1_get_payment_amount (param i64) (result i32 i64)))
    (import "env1" "tx" (func $env1_tx (param i32 i32) (result i32 i32 i32)))

    (import "env0" "set_storage_int" (func $set_storage_int (param i32 i32 i64) (result i32)))
    (import "env0" "set_storage_binary" (func $set_storage_binary (param i32 i32 i32 i32) (result i32)))

    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "env0_get_tx_sender") (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $env0_get_tx_sender)

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

    (func (export "env0_get_payments") (result i32)
        (local $number i32) (local $error i32)
        (block $code
            (call $env0_get_payments)

            (local.set $number)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_int
                        (i32.const 0)
                        (i32.const 6)
                        (i64.extend_i32_u
                            (local.get $number)
                        )
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "env1_get_payments") (result i32)
        (local $number i64) (local $error i32)
        (block $code
            (call $env1_get_payments)

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

    (func (export "env0_get_payment_asset_id") (param $p0 i64) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $env0_get_payment_asset_id
                (i32.wrap_i64
                    (local.get $p0)
                )
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

    (func (export "env1_get_payment_asset_id") (param $p0 i64) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $env1_get_payment_asset_id
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

    (func (export "env0_get_payment_amount") (param $p0 i64) (result i32)
        (local $number i64) (local $error i32)
        (block $code
            (call $env0_get_payment_amount
                (i32.wrap_i64
                    (local.get $p0)
                )
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

    (func (export "env1_get_payment_amount") (param $p0 i64) (result i32)
        (local $number i64) (local $error i32)
        (block $code
            (call $env1_get_payment_amount
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

    (func (export "env1_tx") (param $p0 i32) (param $p1 i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $env1_tx
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
