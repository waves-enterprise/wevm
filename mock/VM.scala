class VM {
    @native def runContract(bytecode: Array[Byte], func_name: String, func_args: Array[Byte], callback: VM): Int

    @native def validateBytecode(bytecode: Array[Byte]): Int

    def getBytecode(name: String): Array[Byte] = {
        println(s"Scala: Name contract: $name")
        // (module
        //     (func (export "_constructor"))
        //     (func (export "run") (param $p0 i64) (result i32)
        //         i32.const 2
        //         (i32.wrap_i64
        //             (local.get $p0))
        //         i32.add))
        Array[Byte](
            0, 97, 115, 109, 1, 0, 0, 0, 1, 9, 2, 96, 0, 0, 96, 1, 126, 1, 127, 3, 3, 2,
            0, 1, 7, 22, 2, 12, 95, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
            0, 0, 3, 114, 117, 110, 0, 1, 10, 13, 2, 2, 0, 11, 8, 0, 65, 2, 32, 0, 167.toByte,
            106, 11, 0, 14, 4, 110, 97, 109, 101, 2, 7, 1, 1, 1, 0, 2, 112, 48
        )
    }
}

object VM {
    def main(args: Array[String]): Unit = {
        System.loadLibrary("wevm")

        // (module
        //     (type $t0 (func (param i32 i32 i32 i32 i32 i32) (result i32)))
        //     (type $t1 (func (result i32)))
        //     (import "env0" "call_contract" (func $call (type $t0)))
        //     (import "env" "memory" (memory $env.memory 2 16))
        //
        //     (func (export "_constructor"))
        //     (func $run (export "run") (type $t1) (result i32)
        //         (call $call
        //             (i32.const 0)
        //             (i32.const 3)
        //             (i32.const 3)
        //             (i32.const 3)
        //             (i32.const 6)
        //             (i32.const 20)))
        //
        //     (data $d0 (i32.const 0) "two")
        //     (data $d1 (i32.const 3) "run")
        //     (data $d2 (i32.const 6) "\01\00\00\00\00\00\00\00\00\00\00\02")
        // )
        val bytecode = Array[Byte](
            0, 97, 115, 109, 1, 0, 0, 0, 1, 18, 3, 96, 6, 127, 127, 127, 127, 127, 127, 1,
            127, 96, 0, 1, 127, 96, 0, 0, 2, 37, 2, 4, 101, 110, 118, 48, 13, 99, 97, 108, 108,
            95, 99, 111, 110, 116, 114, 97, 99, 116, 0, 0, 3, 101, 110, 118, 6, 109, 101, 109,
            111, 114, 121, 2, 1, 2, 16, 3, 3, 2, 2, 1, 7, 22, 2, 12, 95, 99, 111, 110, 115, 116,
            114, 117, 99, 116, 111, 114, 0, 1, 3, 114, 117, 110, 0, 2, 10, 21, 2, 2, 0, 11, 16,
            0, 65, 0, 65, 3, 65, 3, 65, 3, 65, 6, 65, 12, 16, 0, 11, 11, 34, 3, 0, 65, 0, 11, 3,
            116, 119, 111, 0, 65, 3, 11, 3, 114, 117, 110, 0, 65, 6, 11, 12, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 2, 0, 60, 4, 110, 97, 109, 101, 1, 12, 2, 0, 4, 99, 97, 108, 108, 2, 3,
            114, 117, 110, 4, 9, 2, 0, 2, 116, 48, 1, 2, 116, 49, 6, 13, 1, 0, 10, 101, 110, 118,
            46, 109, 101, 109, 111, 114, 121, 9, 13, 3, 0, 2, 100, 48, 1, 2, 100, 49, 2, 2, 100, 50
        )

        val wrongBytecode = Array[Byte](
            0, 14, 21, 1, 2
        )

        val vm = new VM
        val isCorrect = vm.validateBytecode(bytecode) == 0
        val isIncorrect = vm.validateBytecode(wrongBytecode) == 100 // ExecutableError::InvalidBytecode
        println(s"bytecode is correct: $isCorrect")
        println(s"wrong bytecode is incorrect: $isIncorrect")

        val result = vm.runContract(bytecode, "run", Array[Byte](), new VM)
        println(s"run contract result: $result")
    }
}
