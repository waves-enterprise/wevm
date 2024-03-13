package com.wavesenterprise.wasm.core

import com.github.sbt.jni.syntax.NativeLoader

class WASMExecutor extends NativeLoader("wevm") {
  @native def runContract(
      contractId: Array[Byte],
      bytecode: Array[Byte],
      funcName: String,
      params: Array[Byte],
      fuelLimit: Long,
      callback: WASMService
  ): Int

  @native def validateBytecode(bytecode: Array[Byte]): Int
}
