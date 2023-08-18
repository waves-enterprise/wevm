package com.wavesenterprise.wasm.core

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
  }
}
