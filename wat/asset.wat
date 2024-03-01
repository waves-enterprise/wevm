(module
    (import "env" "memory" (memory 2 16))

    (import "env1" "get_balance" (func $get_balance (param i32 i32 i32 i32 i32 i32) (result i32 i64)))
    (import "env1" "transfer" (func $transfer (param i32 i32 i32 i32 i32 i32 i64) (result i32)))
    (import "env1" "issue" (func $issue (param i32 i32 i32 i32 i64 i64 i32) (result i32 i32 i32)))
    (import "env0" "burn" (func $burn (param i32 i32 i64) (result i32)))
    (import "env0" "reissue" (func $reissue (param i32 i32 i64 i32) (result i32)))

    (import "env0" "base_58" (func $base_58 (param i32 i32) (result i32 i32 i32)))
    (import "env0" "set_storage_int" (func $set_storage_int (param i32 i32 i64) (result i32)))

    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "asset") (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
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

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (call $burn
                (local.get $offset)
                (local.get $length)
                (i64.const 42)
            )

            (br_if $code
                (local.tee $error)
            )

            (local.set $error
                (call $reissue
                    (local.get $offset)
                    (local.get $length)
                    (i64.const 24)
                    (i32.const 1)
                )
            )
        )

        (local.get $error)
    )

    (func (export "get_balance") (param $p0 i32) (param $p1 i32) (result i32)
        (local $balance i64) (local $error i32)
        (block $code
            (call $get_balance
                (i32.const 0)
                (i32.const 0)
                (local.get $p0)
                (local.get $p1)
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
                    (i32.const 6)
                    (local.get $balance)
                )
            )
        )

        (local.get $error)
    )

    (func (export "transfer") (result i32)
        (local $offset i32) (local $length i32) (local $balance i64) (local $error i32)
        (block $code
            (call $base_58
                (i32.const 20)  ;; Offset address
                (i32.const 35) ;; Length address
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (call $get_balance
                (i32.const 0)
                (i32.const 0)
                (i32.const 0)
                (i32.const 0)
                (i32.const 0) ;; Type - Account
                (i32.const 1) ;; Version - Address
            )

            (local.set $balance)

            (br_if $code
                (local.tee $error)
            )

            (local.set $error
                (i32.const 300)
            )

            (br_if $code
                (i64.lt_s
                    (local.get $balance)
                    (i64.const 43)
                )
            )

            (local.set $error
                (call $transfer
                    (i32.const 0)
                    (i32.const 0)
                    (local.get $offset)
                    (local.get $length)
                    (i32.const 0) ;; Type - Account
                    (i32.const 1) ;; Version - Address
                    (i64.const 42)
                )
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 55))

    ;; Name
    (data (i32.const 0) "TEST")
    ;; Description
    (data (i32.const 4) "Test asset")
    ;; Key
    (data (i32.const 14) "result")
    ;; Address
    (data (i32.const 20) "3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU")
)
