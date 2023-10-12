(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "base_58" (func $base_58 (param i32 i32) (result i32 i32 i32)))
    (import "env0" "to_base_58_string" (func $to_base_58_string (param i32 i32) (result i32 i32 i32)))

    (import "env0" "set_storage_string" (func $set_storage_string (param i32 i32 i32 i32) (result i32)))

    (func (export "_constructor") (param $address_offset i32) (param $address_length i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $base_58
                (local.get $address_offset)
                (local.get $address_length)
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
                        (i32.const 7) ;; Key length
                        (local.get $offset)
                        (local.get $length)
                    )
                )
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 7))

    ;; Key
    (data (i32.const 0) "address")
)
