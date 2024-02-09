(module
    (import "env" "memory" (memory 2 16))

    (import "env1" "issue" (func $issue (param i32 i32 i32 i32 i64 i64 i32) (result i32 i32 i32)))
    (import "env0" "burn" (func $burn (param i32 i32 i64) (result i32)))
    (import "env0" "reissue" (func $reissue (param i32 i32 i64 i32) (result i32)))

    (import "env1" "get_balance" (func $get_address_balance (param i32 i32 i32 i32 i32 i32) (result i32 i64)))
    (import "env0" "set_storage_int" (func $set_storage_int (param i32 i32 i64) (result i32)))

    (func (export "_constructor") (result i32)
        (local $a_offset i32) (local $a_length i32) (local $error i32)
        (block $code
            (call $issue
                (i32.const 0)   ;; Name offset
                (i32.const 4)   ;; Name length
                (i32.const 4)   ;; Description offset
                (i32.const 10)  ;; Description length
                (i64.const 100) ;; Quantity
                (i64.const 8)   ;; Decimals
                (i32.const 1)   ;; Is reissuable
            )

            (local.set $a_length)
            (local.set $a_offset)

            (br_if $code
                (local.tee $error)
            )

            (call $burn
                (local.get $a_offset)
                (local.get $a_length)
                (i64.const 42)
            )

            (br_if $code
                (local.tee $error)
            )

            (local.set $error
                (call $reissue
                    (local.get $a_offset)
                    (local.get $a_length)
                    (i64.const 24)
                    (i32.const 1)
                )
            )
        )

        (local.get $error)
    )

    (func (export "check_balance") (param $address_offset i32) (param $address_length i32) (result i32)
        (local $balance i64) (local $error i32)
        (block $code
            (call $get_address_balance
                (i32.const 0)
                (i32.const 0)
                (local.get $address_offset)
                (local.get $address_length)
                (i32.const 0) ;; Type - Account
                (i32.const 1) ;; Version - Address
            )

            (local.set $balance)

            (br_if $code
                (local.tee $error)
            )

            (local.set $error
                (call $set_storage_int
                    (i32.const 14)
                    (i32.const 7)
                    (local.get $balance)
                )
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 21))

    ;; Name
    (data (i32.const 0) "TEST")
    ;; Description
    (data (i32.const 4) "Test asset")
    ;; Key balance
    (data (i32.const 14) "balance")
)
