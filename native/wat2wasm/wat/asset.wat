(module
    (import "env" "memory" (memory 2 16))

    (import "env0" "issue" (func $issue (param i32 i32 i32 i32 i64 i32 i32) (result i32 i32 i32)))
    (import "env0" "burn" (func $burn (param i32 i32 i64) (result i32)))
    (import "env0" "reissue" (func $reissue (param i32 i32 i64 i32) (result i32)))

    (func (export "_constructor") (result i32)
        (local $a_offset i32) (local $a_length i32) (local $error i32)
        (block $code
            (call $issue
                (i32.const 0)   ;; Name offset
                (i32.const 4)   ;; Name length
                (i32.const 4)   ;; Description offset
                (i32.const 10)  ;; Description length
                (i64.const 100) ;; Quantity
                (i32.const 8)   ;; Decimals
                (i32.const 1)   ;; Is reissuable
            )

            (local.set $a_length)
            (local.set $a_offset)

            (br_if $code
                (local.tee $error)
            )

            (call $burn
                (local.get $a_offset)
                (local.get $a_length)
                (i64.const 42)
            )

            (br_if $code
                (local.tee $error)
            )

            (local.set $error
                (call $reissue
                    (local.get $a_offset)
                    (local.get $a_length)
                    (i64.const 24)
                    (i32.const 1)
                )
            )
        )

        (local.get $error)
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 14))

    ;; Name
    (data (i32.const 0) "TEST")
    ;; Description
    (data (i32.const 4) "Test asset")
)
