(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "parse_int" (func $parse_int (param i32 i32) (result i32 i64)))
    (import "env0" "parse_bool" (func $parse_bool (param i32 i32) (result i32 i32)))
    (import "env0" "to_bytes" (func $to_bytes (param i64) (result i32 i32 i32)))
    (import "env0" "to_int" (func $to_int (param i32 i32) (result i32 i64)))
    (import "env0" "to_string_bool" (func $to_string_bool (param i32) (result i32 i32 i32)))
    (import "env0" "to_string_int" (func $to_string_int (param i64) (result i32 i32 i32)))

    (import "env0" "set_storage_int" (func $set_storage_int (param i32 i32 i64) (result i32)))
    (import "env0" "set_storage_bool" (func $set_storage_bool (param i32 i32 i32) (result i32)))
    (import "env0" "set_storage_binary" (func $set_storage_binary (param i32 i32 i32 i32) (result i32)))
    (import "env0" "set_storage_string" (func $set_storage_string (param i32 i32 i32 i32) (result i32)))

    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "parse_int") (param $p0 i32) (param $p1 i32) (result i32)
        (local $result i64) (local $error i32)
        (block $code
            (call $parse_int
                (local.get $p0)
                (local.get $p1)
            )

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

    (func (export "parse_bool") (param $p0 i32) (param $p1 i32) (result i32)
        (local $result i32) (local $error i32)
        (block $code
            (call $parse_bool
                (local.get $p0)
                (local.get $p1)
            )

            (local.set $result)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_bool
                        (i32.const 0)
                        (i32.const 6)
                        (local.get $result)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "to_bytes") (param $p0 i64) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $to_bytes
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

    (func (export "to_int") (param $p0 i32) (param $p1 i32) (result i32)
        (local $result i64) (local $error i32)
        (block $code
            (call $to_int
                (local.get $p0)
                (local.get $p1)
            )

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

    (func (export "to_string_bool") (param $p0 i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $to_string_bool
                (local.get $p0)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_string
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

    (func (export "to_string_int") (param $p0 i64) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $to_string_int
                (local.get $p0)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_string
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
