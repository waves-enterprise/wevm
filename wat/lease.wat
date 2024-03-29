(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "base_58" (func $base_58 (param i32 i32) (result i32 i32 i32)))
    (import "env0" "lease_address" (func $lease_address (param i32 i32 i64) (result i32 i32 i32)))
    (import "env0" "lease_alias" (func $lease_alias (param i32 i32 i64) (result i32 i32 i32)))
    (import "env0" "cancel_lease" (func $cancel_lease (param i32 i32) (result i32)))

    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "lease_address") (result i32)
        (local $address_offset i32) (local $address_length i32) (local $lease_offset i32) (local $lease_length i32) (local $error i32)
        (block $code
            (call $base_58
                (i32.const 0)  ;; Offset address
                (i32.const 35) ;; Length address
            )

            (local.set $address_length)
            (local.set $address_offset)

            (br_if $code
                (local.tee $error)
            )

            (call $lease_address
                (local.get $address_offset)
                (local.get $address_length)
                (i64.const 42)
            )

            (local.set $lease_length)
            (local.set $lease_offset)

            (br_if $code
                (local.tee $error)
            )

            (local.set $error
                (call $cancel_lease
                    (local.get $lease_offset)
                    (local.get $lease_length)
                )
            )
        )

        (local.get $error)
    )

    (func (export "lease_alias") (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $lease_alias
                (i32.const 35) ;; Offset alias
                (i32.const 5)  ;; Length alias
                (i64.const 24)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (local.set $error
                (call $cancel_lease
                    (local.get $offset)
                    (local.get $length)
                )
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 40))

    ;; Address
    (data (i32.const 0) "3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3")

    ;; Alias
    (data (i32.const 35) "miner")
)
