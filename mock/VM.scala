class VM {
    @native def runContract(bytecode: Array[Byte], func_name: String, func_args: Array[String], callback: VM): Int

    def getBytecode(name: String): Array[Byte] = {
        println(s"Scala: Name contract: $name")
        // (module
        //   (type $t0 (func (result i32)))
        //   (func $run (export "run") (type $t0) (result i32)
        //     (i32.add
        //       (i32.const 2)
        //       (i32.const 2)
        //     )
        //   )
        // )
        Array[Byte](0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 127, 3, 2, 1, 0, 7, 7, 1, 3, 114, 117, 110, 0, 0, 10, 9, 1, 7, 0, 65, 2, 65, 2, 106, 11)
    }
}

object VM {
    def main(args: Array[String]): Unit = {
        System.loadLibrary("wevm")

        // (module
        //   (type $t0 (func (param i32 i32 i32 i32 i32 i32) (result i32)))
        //   (type $t1 (func (result i32)))
        //   (import "env" "call_contract" (func $call (type $t0)))
        //   (import "env" "memory" (memory $env.memory 1 1))
        //   (func $run (export "run") (type $t1) (result i32)
        //     (call $call
        //       (i32.const 0)
        //       (i32.const 3)
        //       (i32.const 3)
        //       (i32.const 3)
        //       (i32.const 6)
        //       (i32.const 4)))
        //
        //   (data $d0 (i32.const 0) "two")
        //   (data $d1 (i32.const 3) "run")
        //   (data $d2 (i32.const 6) "\01\02\03\04")
        // )
        val bytecode = Array[Byte](0, 97, 115, 109, 1, 0, 0, 0, 1, 15, 2, 96, 6, 127, 127, 127, 127, 127, 127, 1, 127, 96, 0, 1, 127, 2, 36, 2, 3, 101, 110, 118, 13, 99, 97, 108, 108, 95, 99, 111, 110, 116, 114, 97, 99, 116, 0, 0, 3, 101, 110, 118, 6, 109, 101, 109, 111, 114, 121, 2, 1, 1, 1, 3, 2, 1, 1, 7, 7, 1, 3, 114, 117, 110, 0, 1, 10, 18, 1, 16, 0, 65, 0, 65, 3, 65, 3, 65, 3, 65, 6, 65, 4, 16, 0, 11, 11, 26, 3, 0, 65, 0, 11, 3, 116, 119, 111, 0, 65, 3, 11, 3, 114, 117, 110, 0, 65, 6, 11, 4, 1, 2, 3, 4, 0, 14, 4, 110, 97, 109, 101, 1, 7, 1, 0, 4, 99, 97, 108, 108)
        val funcName = "run"
        val funcArgs = Array[String]()

        val vm = new VM
        val result = vm.runContract(bytecode, funcName, funcArgs, new VM)
        println(s"Result: $result")
    }
}
