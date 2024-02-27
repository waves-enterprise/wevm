(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "binary_equals" (func $binary_equals (param i32 i32 i32 i32) (result i32 i32)))
    (import "env0" "string_equals" (func $string_equals (param i32 i32 i32 i32) (result i32 i32)))
    (import "env0" "join" (func $join (param i32 i32 i32 i32) (result i32 i32 i32)))

    (import "env0" "to_le_bytes" (func $to_le_bytes (param i32 i32) (result i32 i32 i32)))
    (import "env0" "set_storage_int" (func $set_storage_int (param i32 i32 i64) (result i32)))

    (func (export "_constructor") (result i32)
        (local $offset i32) (local $length i32) (local $result i32) (local $error i32)
        (block $code
            ;; Binary concatenation
            (call $join
                (i32.const 0)
                (i32.const 2)
                (i32.const 2)
                (i32.const 2)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (call $binary_equals
                (i32.const 0)
                (i32.const 4)
                (local.get $offset)
                (local.get $length)
            )

            (local.set $result)

            (br_if $code
                (local.tee $error)
            )

            (block $require
                (br_if $require
                    (local.get $result)
                )
                (return
                    (i32.const 300)
                )
            )

            ;; String concatenation
            (call $join
                (i32.const 4)
                (i32.const 3)
                (i32.const 7)
                (i32.const 3)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (call $string_equals
                (i32.const 4)
                (i32.const 6)
                (local.get $offset)
                (local.get $length)
            )

            (local.set $result)

            (br_if $code
                (local.tee $error)
            )

            (block $require
                (br_if $require
                    (local.get $result)
                )
                (return
                    (i32.const 300)
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
                        (i32.const 10)
                        (i32.const 7)
                        (local.get $result)
                    )
                )
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 17))

    ;; Binary
    (data (i32.const 0) "\01\02")
    (data (i32.const 2) "\03\04")
    ;; String
    (data (i32.const 4) "one")
    (data (i32.const 7) "two")
    ;; Key
    (data (i32.const 10) "integer")
)
