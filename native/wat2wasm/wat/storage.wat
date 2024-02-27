(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "set_storage_int" (func $set_storage_int (param i32 i32 i64) (result i32)))
    (import "env0" "set_storage_bool" (func $set_storage_bool (param i32 i32 i32) (result i32)))
    (import "env0" "set_storage_binary" (func $set_storage_binary (param i32 i32 i32 i32) (result i32)))
    (import "env0" "set_storage_string" (func $set_storage_string (param i32 i32 i32 i32) (result i32)))

    (import "env0" "get_storage_int" (func $get_storage_int (param i32 i32 i32 i32) (result i32 i64)))
    (import "env0" "get_storage_bool" (func $get_storage_bool (param i32 i32 i32 i32) (result i32 i32)))

    (import "env0" "caller" (func $caller (result i32 i32 i32)))

    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "save") (param $p_int i64) (param $p_bool i32) (param $p_binary_offset i32) (param $p_binary_length i32) (param $p_string_offset i32) (param $p_string_length i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)

        (block $code
            (br_if $code
                (local.tee $error
                    (call $set_storage_int
                        (i32.const 0)  ;; Key offset
                        (i32.const 11) ;; Key length
                        (local.get $p_int)
                    )
                )
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_bool
                        (i32.const 11) ;; Key offset
                        (i32.const 11) ;; Key length
                        (local.get  $p_bool)
                    )
                )
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_binary
                        (i32.const 22) ;; Key offset
                        (i32.const 10) ;; Key length
                        (local.get $p_binary_offset)
                        (local.get $p_binary_length)    
                    )
                )
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_string
                        (i32.const 32) ;; Key offset
                        (i32.const 10) ;; Key length
                        (local.get $p_string_offset)
                        (local.get $p_string_length)
                    )
                )
            )

            (call $caller)

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_binary
                        (i32.const 42) ;; Key offset
                        (i32.const 6)  ;; Key length
                        (local.get $offset)
                        (local.get $length)
                    )
                )
            )

            (call $get_storage_int
                (i32.const 0)
                (i32.const 0)
                (i32.const 0)
                (i32.const 11)
            )
            (local.set $p_int)

            (br_if $code
                (local.tee $error)
            )

            (local.set $error
                (i32.const 300)
            )
            (br_if $code
                (i64.ne
                    (local.get $p_int)
                    (i64.const 42)
                )
            )

            (call $get_storage_bool
                (i32.const 0)
                (i32.const 0)
                (i32.const 11)
                (i32.const 11)
            )
            (local.set $p_bool)

            (br_if $code
                (local.tee $error)
            )

            (local.set $error
                (i32.const 300)
            )
            (br_if $code
                (i32.ne
                    (local.get $p_bool)
                    (i32.const 1)
                )
            )

            (local.set $error
                (i32.const 0) ;; Resetting the errors
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 48))

    ;; Keys
    (data (i32.const 0) "integer_key")
    (data (i32.const 11) "boolean_key")
    (data (i32.const 22) "binary_key")
    (data (i32.const 32) "string_key")

    (data (i32.const 42) "caller")
)
