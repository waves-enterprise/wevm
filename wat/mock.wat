(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "caller" (func $caller (result i32 i32 i32)))
    (import "env0" "set_storage_binary" (func $set_storage_binary (param i32 i32 i32 i32) (result i32)))

    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "caller") (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $caller)

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_binary
                        (i32.const 0) ;; Key offset
                        (i32.const 6)  ;; Key length
                        (local.get $offset)
                        (local.get $length)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "infinite_loop") (result i32)
        (loop $loop (result i32)
            (br $loop)
        )
    )

    (func (export "recursion") (result i32)
        (call $one)
        (i32.const 0)
    )

    (func $one (call $two))
    (func $two (call $one))

    (global $__heap_base (export "__heap_base") i32 (i32.const 6))

    ;; Key
    (data (i32.const 0) "result")
)
