(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "contains_key" (func $contains_key (param i32 i32 i32 i32) (result i32 i32)))

    (import "env0" "set_storage_int" (func $set_storage_int (param i32 i32 i64) (result i32)))
    (import "env0" "set_storage_bool" (func $set_storage_bool (param i32 i32 i32) (result i32)))
    (import "env0" "set_storage_binary" (func $set_storage_binary (param i32 i32 i32 i32) (result i32)))
    (import "env0" "set_storage_string" (func $set_storage_string (param i32 i32 i32 i32) (result i32)))

    (import "env0" "get_storage_int" (func $get_storage_int (param i32 i32 i32 i32) (result i32 i64)))
    (import "env0" "get_storage_bool" (func $get_storage_bool (param i32 i32 i32 i32) (result i32 i32)))
    (import "env0" "get_storage_binary" (func $get_storage_binary (param i32 i32 i32 i32) (result i32 i32 i32)))
    (import "env0" "get_storage_string" (func $get_storage_string (param i32 i32 i32 i32) (result i32 i32 i32)))

    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "contains_key") (result i32)
        (local $result i32) (local $error i32)
        (block $code
            (call $contains_key
                (i32.const 0)
                (i32.const 0)
                (i32.const 0)
                (i32.const 7)
            )

            (local.set $result)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_bool
                        (i32.const 26)
                        (i32.const 6)
                        (local.get $result)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "set_storage") (param $p0 i64) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32) (param $p5 i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)

        (block $code
            (br_if $code
                (local.tee $error
                    (call $set_storage_int
                        (i32.const 0)  ;; Key offset
                        (i32.const 7) ;; Key length
                        (local.get $p0)
                    )
                )
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_bool
                        (i32.const 7) ;; Key offset
                        (i32.const 7) ;; Key length
                        (local.get  $p1)
                    )
                )
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_binary
                        (i32.const 14) ;; Key offset
                        (i32.const 6) ;; Key length
                        (local.get $p2)
                        (local.get $p3)
                    )
                )
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_string
                        (i32.const 20) ;; Key offset
                        (i32.const 6) ;; Key length
                        (local.get $p4)
                        (local.get $p5)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "get_storage_int") (param $p0 i32) (param $p1 i32) (result i32)
        (local $integer i64) (local $error i32)
        (block $code
            (call $get_storage_int
                (i32.const 0)
                (i32.const 0)
                (local.get $p0)
                (local.get $p1)
            )

            (local.set $integer)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_int
                        (i32.const 26)
                        (i32.const 6)
                        (local.get $integer)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "get_storage_bool") (param $p0 i32) (param $p1 i32) (result i32)
        (local $boolean i32) (local $error i32)
        (block $code
            (call $get_storage_bool
                (i32.const 0)
                (i32.const 0)
                (local.get $p0)
                (local.get $p1)
            )

            (local.set $boolean)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $set_storage_bool
                        (i32.const 26)
                        (i32.const 6)
                        (local.get $boolean)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "get_storage_binary") (param $p0 i32) (param $p1 i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $get_storage_binary
                (i32.const 0)
                (i32.const 0)
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
                        (i32.const 26)
                        (i32.const 6)
                        (local.get $offset)
                        (local.get $length)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "get_storage_string") (param $p0 i32) (param $p1 i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $get_storage_string
                (i32.const 0)
                (i32.const 0)
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
                    (call $set_storage_string
                        (i32.const 26)
                        (i32.const 6)
                        (local.get $offset)
                        (local.get $length)
                    )
                )
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 32))

    ;; Keys
    (data (i32.const 0) "integer")
    (data (i32.const 7) "boolean")
    (data (i32.const 14) "binary")
    (data (i32.const 20) "string")
    (data (i32.const 26) "result")
)
