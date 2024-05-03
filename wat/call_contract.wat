(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "base_58" (func $base_58 (param i32 i32) (result i32 i32 i32)))

    (import "env0" "call_arg_int" (func $call_arg_int (param i64)))
    (import "env0" "call_arg_bool" (func $call_arg_bool (param i32)))
    (import "env0" "call_arg_binary" (func $call_arg_binary (param i32 i32) (result i32)))
    (import "env0" "call_arg_string" (func $call_arg_string (param i32 i32) (result i32)))

    (import "env0" "call_payment" (func $call_payment (param i32 i32 i64) (result i32)))

    (import "env0" "call_contract" (func $call_contract (param i32 i32 i32 i32) (result i32)))
    (import "env0" "call_contract_params" (func $call_contract_params (param i32 i32 i32 i32 i32 i32) (result i32)))

    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "call_contract") (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32) (param $p5 i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)

        (block $code
            (call $call_arg_int
                (i64.const 42)
            )

            (call $call_arg_bool
                (i32.const  1)
            )

            (br_if $code
                (local.tee $error
                    (call $call_arg_binary
                        (i32.const 0)
                        (i32.const 2)
                    )
                )
            )

            (br_if $code
                (local.tee $error
                    (call $call_arg_string
                        (i32.const 2)
                        (i32.const 4)
                    )
                )
            )

            (call $base_58
                (local.get $p4)
                (local.get $p5)
            )

            (local.set $length)
            (local.set $offset)

            (br_if $code
                (local.tee $error)
            )

            (br_if $code
                (local.tee $error
                    (call $call_payment
                        (i32.const 0)
                        (i32.const 0)
                        (i64.const 4200000000)
                    )
                )
            )

            (br_if $code
                (local.tee $error
                    (call $call_payment
                        (local.get $offset)
                        (local.get $length)
                        (i64.const 2400000000)
                    )
                )
            )

            (call $base_58
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
                    (call $call_contract
                        (local.get $offset)
                        (local.get $length)
                        (local.get $p2)
                        (local.get $p3)
                    )
                )
            )
        )

        (local.get $error)
    )

    (func (export "call_contract_params") (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32) (param $p5 i32) (result i32)
        (local $offset i32) (local $length i32) (local $error i32)

        (block $code
            (call $base_58
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
                    (call $call_contract_params
                        (local.get $offset)
                        (local.get $length)
                        (local.get $p2)
                        (local.get $p3)
                        (local.get $p4)
                        (local.get $p5)
                    )
                )
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 6))

    ;; Args
    (data (i32.const 0) "\00\01")
    (data (i32.const 2) "test")
)
