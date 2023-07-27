package com.wavesenterprise.wasm

import com.github.sbt.jni.syntax.NativeLoader

class WASMExecutor extends NativeLoader("wevm") {
  @native def runContract(
      bytecode: Array[Byte],
      funcName: String,
      funcArgs: Array[Byte],
      callback: WASMService
  ): Int

  @native def validateBytecode(bytecode: Array[Byte]): Int
}
