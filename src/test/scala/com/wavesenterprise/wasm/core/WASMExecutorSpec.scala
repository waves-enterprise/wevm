package com.wavesenterprise.wasm.core

import com.google.common.io.{ByteArrayDataOutput, ByteStreams}
import com.wavesenterprise.state.{ByteStr, DataEntry, IntegerDataEntry, BooleanDataEntry, BinaryDataEntry, StringDataEntry}
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class WASMExecutorSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  "WASMExecutor" - {
    "validate bytecode" in {
      val bytecode = getClass.getResourceAsStream("/mock.wasm").readAllBytes()

      val wrongBytecode = Array[Byte](
        0, 14, 21, 1, 2
      )

      executor.validateBytecode(bytecode) shouldBe 0
      executor.validateBytecode(wrongBytecode) shouldBe 100
    }

    "infinite_loop" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/mock.wasm").readAllBytes()

      executor.runContract(contractId, bytecode, "infinite_loop", Array[Byte](), fuelLimit, service) shouldBe 111
    }

    "recursion" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/mock.wasm").readAllBytes()

      executor.runContract(contractId, bytecode, "recursion", Array[Byte](), fuelLimit, service) shouldBe 111
    }
  }
}
