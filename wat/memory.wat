(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "binary_equals" (func $binary_equals (param i32 i32 i32 i32) (result i32 i32)))
    (import "env0" "string_equals" (func $string_equals (param i32 i32 i32 i32) (result i32 i32)))
    (import "env0" "join" (func $join (param i32 i32 i32 i32) (result i32 i32 i32)))
    (import "env0" "contains" (func $contains (param i32 i32 i32 i32) (result i32 i32)))
    (import "env0" "drop" (func $drop (param i32 i32 i64) (result i32 i32 i32)))
    (import "env0" "drop_right" (func $drop_right (param i32 i32 i64) (result i32 i32 i32)))
    (import "env0" "index_of" (func $index_of (param i32 i32 i32 i32) (result i32 i64)))
    (import "env0" "last_index_of" (func $last_index_of (param i32 i32 i32 i32) (result i32 i64)))
    (import "env0" "take" (func $take (param i32 i32 i64) (result i32 i32 i32)))
    (import "env0" "take_right" (func $take_right (param i32 i32 i64) (result i32 i32 i32)))

    (import "env0" "set_storage_int" (func $set_storage_int (param i32 i32 i64) (result i32)))
    (import "env0" "set_storage_bool" (func $set_storage_bool (param i32 i32 i32) (result i32)))
    (import "env0" "set_storage_string" (func $set_storage_string (param i32 i32 i32 i32) (result i32)))


    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "binary_equals") (result i32)
        (local $offset i32) (local $length i32) (local $result i32) (local $error i32)
        (block $code
            (call $join
                (i32.const 6)
                (i32.const 2)
                (i32.const 8)
                (i32.const 2)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (call $binary_equals
                (i32.const 6)
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
        )

        (local.get $error)
    )

    (func (export "string_equals") (result i32)
        (local $offset i32) (local $length i32) (local $result i32) (local $error i32)
        (block $code
            (call $join
                (i32.const 10)
                (i32.const 3)
                (i32.const 13)
                (i32.const 3)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (call $string_equals
                (i32.const 10)
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

    (func (export "contains") (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (result i32)
        (local $result i32) (local $error i32)
        (block $code
            (call $contains
                (local.get $p0)
                (local.get $p1)
                (local.get $p2)
                (local.get $p3)
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

    (func (export "drop") (param $p0 i32) (param $p1 i32) (param $p2 i64) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $drop
                (local.get $p0)
                (local.get $p1)
                (local.get $p2)
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

    (func (export "drop_right") (param $p0 i32) (param $p1 i32) (param $p2 i64) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $drop_right
                (local.get $p0)
                (local.get $p1)
                (local.get $p2)
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

    (func (export "index_of") (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (result i32)
        (local $result i64) (local $error i32)
        (block $code
            (call $index_of
                (local.get $p0)
                (local.get $p1)
                (local.get $p2)
                (local.get $p3)
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

    (func (export "last_index_of") (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (result i32)
        (local $result i64) (local $error i32)
        (block $code
            (call $last_index_of
                (local.get $p0)
                (local.get $p1)
                (local.get $p2)
                (local.get $p3)
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

    (func (export "take") (param $p0 i32) (param $p1 i32) (param $p2 i64) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $take
                (local.get $p0)
                (local.get $p1)
                (local.get $p2)
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

    (func (export "take_right") (param $p0 i32) (param $p1 i32) (param $p2 i64) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)
        (block $code
            (call $take_right
                (local.get $p0)
                (local.get $p1)
                (local.get $p2)
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
