(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "get_block_timestamp" (func $get_block_timestamp (result i32 i64)))
    (import "env0" "get_block_height" (func $get_block_height (result i32 i64)))
    (import "env1" "block" (func $block (param i32 i32) (result i32 i32 i32)))

    (import "env0" "set_storage_int" (func $set_storage_int (param i32 i32 i64) (result i32)))

    (import "env0" "to_le_bytes" (func $to_le_bytes (param i32 i32) (result i32 i32 i32)))

    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "get_block_timestamp") (result i32)
        (local $timestamp i64) (local $error i32)
        (block $code
            (call $get_block_timestamp)

            (local.set $timestamp)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_int
                        (i32.const 0)
                        (i32.const 6)
                        (local.get $timestamp)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "get_block_height") (result i32)
        (local $height i64) (local $error i32)
        (block $code
            (call $get_block_height)

            (local.set $height)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_int
                        (i32.const 0)
                        (i32.const 6)
                        (local.get $height)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "block") (param $p0 i32) (param $p1 i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $block
                (local.get $p0)
                (local.get $p1)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (call $to_le_bytes
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
                    (call $set_storage_int
                        (i32.const 0)
                        (i32.const 6)
                        (i64.load
                            (local.get $offset)
                        )
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
