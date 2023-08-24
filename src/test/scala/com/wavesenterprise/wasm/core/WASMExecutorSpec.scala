package com.wavesenterprise.wasm.core

import com.wavesenterprise.state.ByteStr
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class WASMExecutorSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor
  val service = new WASMServiceMock

  "WASMExecutor" - {
    "validate bytecode" in {
      val bytecode = getClass.getResourceAsStream("/mock.wasm").readAllBytes()

      val wrongBytecode = Array[Byte](
        0, 14, 21, 1, 2
      )

      executor.validateBytecode(bytecode) shouldBe 0
      executor.validateBytecode(wrongBytecode) shouldBe 100
    }

    "transfer" in {
      val bytecode = getClass.getResourceAsStream("/transfer.wasm").readAllBytes()

      executor.runContract(bytecode, "_constructor", Array[Byte](), service) shouldBe 0

      service.balances("null")(service.contract) shouldBe 9999999958L
      service.balances("null")("3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU") shouldBe 10000000042L
    }

    "storage" in {
      val bytecode = getClass.getResourceAsStream("/storage.wasm").readAllBytes()

      executor.runContract(bytecode, "_constructor", Array[Byte](), service) shouldBe 0

      service.storage("integer_key").value shouldBe 42
      service.storage("boolean_key").value shouldBe true
      service.storage("binary_key").value.equals(ByteStr(Array[Byte](0, 1))) shouldBe true
      service.storage("string_key").value shouldBe "test"
    }
  }
}
