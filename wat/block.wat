(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "get_block_timestamp" (func $get_block_timestamp (result i32 i64)))
    (import "env0" "get_block_height" (func $get_block_height (result i32 i64)))

    (import "env0" "set_storage_int" (func $set_storage_int (param i32 i32 i64) (result i32)))

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

    (global $__heap_base (export "__heap_base") i32 (i32.const 6))

    ;; Key
    (data (i32.const 0) "result")
)
