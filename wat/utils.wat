(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "base_58" (func $base_58 (param i32 i32) (result i32 i32 i32)))
    (import "env0" "to_base_58_string" (func $to_base_58_string (param i32 i32) (result i32 i32 i32)))
    (import "env0" "to_le_bytes" (func $to_le_bytes (param i32 i32) (result i32 i32 i32)))

    (import "env0" "set_storage_int" (func $set_storage_int (param i32 i32 i64) (result i32)))
    (import "env0" "set_storage_string" (func $set_storage_string (param i32 i32 i32 i32) (result i32)))
    (import "env0" "call_contract" (func $call_contract (param i32 i32 i32 i32) (result i32)))

    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "base58") (param $p0 i32) (param $p1 i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $base_58
                (local.get $p0)
                (local.get $p1)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (call $to_base_58_string
                (local.get $offset)
                (local.get $length)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_string
                        (i32.const 0) ;; Key offset
                        (i32.const 6) ;; Key length
                        (local.get $offset)
                        (local.get $length)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "to_le_bytes") (param $p0 i32) (param $p1 i32) (result i32)
        (local $result i64) (local $error i32)
        (block $code
            (call $to_le_bytes
                (local.get $p0)
                (local.get $p1)
            )

            (drop)
            (i64.load)
            (local.set $result)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_int
                        (i32.const 0)
                        (i32.const 6)
                        (local.get $result)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "caller") (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $base_58
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
                    (call $call_contract
                        (local.get $offset)
                        (local.get $length)
                        (local.get $p2)
                        (local.get $p3)
                    )
                )
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 16))

    ;; Key
    (data (i32.const 0) "result")
    ;; Binary
    (data (i32.const 6) "\01\02")
    (data (i32.const 8) "\03\04")
    ;; String
    (data (i32.const 10) "one")
    (data (i32.const 13) "two")
)
