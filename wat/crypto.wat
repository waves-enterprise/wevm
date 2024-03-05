(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "set_storage_bool" (func $set_storage_bool (param i32 i32 i32) (result i32)))
    (import "env0" "set_storage_binary" (func $set_storage_binary (param i32 i32 i32 i32) (result i32)))

    (import "env0" "fast_hash" (func $fast_hash (param i32 i32) (result i32 i32 i32)))
    (import "env0" "secure_hash" (func $secure_hash (param i32 i32) (result i32 i32 i32)))
    (import "env0" "sig_verify" (func $sig_verify (param i32 i32 i32 i32 i32 i32) (result i32 i32)))

    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "fast_hash") (param $p0 i32) (param $p1 i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $fast_hash
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
                    (call $set_storage_binary
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

    (func (export "secure_hash") (param $p0 i32) (param $p1 i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $secure_hash
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
                    (call $set_storage_binary
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

    (func (export "sig_verify") (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32) (param $p5 i32) (result i32)
        (local $result i32) (local $error i32)
        (block $code
            (call $sig_verify
                (local.get $p0)
                (local.get $p1)
                (local.get $p2)
                (local.get $p3)
                (local.get $p4)
                (local.get $p5)
            )

            (local.set $result)

            (br_if $code
                (local.tee $error) 
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_bool
                        (i32.const 0) ;; Key offset
                        (i32.const 6) ;; Key length
                        (local.get $result)
                    )
                )
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 6))

    ;; Keys
    (data (i32.const 0) "result")
)
