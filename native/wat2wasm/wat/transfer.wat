(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "base_58" (func $base_58 (param i32 i32) (result i32 i32 i32)))
    (import "env0" "get_balance" (func $get_balance (param i32 i32 i32 i32) (result i32 i64)))
    (import "env0" "transfer" (func $transfer (param i32 i32 i32 i32 i64) (result i32)))

    (func (export "_constructor") (result i32)
        (local $offset i32) (local $length i32) (local $error i32) (local $balance i64)
        (block $code
            (call $base_58
                (i32.const 0)  ;; Offset address
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
                    (i64.const 42)
                )
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 35))

    ;; Address
    (data (i32.const 0) "3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU")
)
