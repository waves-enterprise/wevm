class VM {
    @native def runContract(bytecode: Array[Byte], func_name: String, func_args: Array[String]): Int
}

object VM {
    def main(args: Array[String]): Unit = {
        System.loadLibrary("wevm")

        val bytecode = Array[Byte](0, 97, 115, 109, 1, 0, 0, 0, 1, 15, 2, 96, 6, 127, 127, 127, 127, 127, 127, 1, 127, 96, 0, 1, 127, 2, 36, 2, 3, 101, 110, 118, 13, 99, 97, 108, 108, 95, 99, 111, 110, 116, 114, 97, 99, 116, 0, 0, 3, 101, 110, 118, 6, 109, 101, 109, 111, 114, 121, 2, 1, 1, 1, 3, 2, 1, 1, 7, 7, 1, 3, 114, 117, 110, 0, 1, 10, 18, 1, 16, 0, 65, 0, 65, 3, 65, 3, 65, 3, 65, 6, 65, 4, 16, 0, 11, 11, 26, 3, 0, 65, 0, 11, 3, 116, 119, 111, 0, 65, 3, 11, 3, 114, 117, 110, 0, 65, 6, 11, 4, 1, 2, 3, 4, 0, 14, 4, 110, 97, 109, 101, 1, 7, 1, 0, 4, 99, 97, 108, 108)
        val funcName = "run"
        val funcArgs = Array[String]()

        val vm = new VM
        val result = vm.runContract(bytecode, funcName, funcArgs)

        println(s"Result: $result")
    }

    def getBytecode(): Array[Byte] = {
        Array[Byte](0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 127, 3, 2, 1, 0, 7, 7, 1, 3, 114, 117, 110, 0, 0, 10, 9, 1, 7, 0, 65, 2, 65, 2, 106, 11)
    }
}