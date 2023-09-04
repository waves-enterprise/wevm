(module
    (import "env" "memory" (memory 2 16))

    (func (export "_constructor") (result i32)
        (i32.const 0)
    )

    (func (export "sum") (param $int i64) (result i32)
        (i32.ne
            (i32.add
                (i32.const 2)
                (i32.wrap_i64
                    (local.get $int)
                )
            )
            (i32.const 4)
        )
    )

    (global $__heap_base (export "__heap_base") i32 (i32.const 0))
)
