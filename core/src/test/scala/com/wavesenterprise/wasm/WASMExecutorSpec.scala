package com.wavesenterprise.wasm

import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class WASMExecutorSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  "WASMExecutor" - {
    "runContract" in {

    }

    "validate bytecode" in {
      val bytecode = Array[Byte](
        0, 97, 115, 109, 1, 0, 0, 0, 1, 18, 3, 96, 6, 127, 127, 127, 127, 127, 127,
        1, 127, 96, 0, 1, 127, 96, 0, 0, 2, 37, 2, 4, 101, 110, 118, 48, 13, 99, 97, 108,
        108, 95, 99, 111, 110, 116, 114, 97, 99, 116, 0, 0, 3, 101, 110, 118, 6, 109, 101,
        109, 111, 114, 121, 2, 1, 1, 1, 3, 3, 2, 2, 1, 7, 22, 2, 12, 95, 99, 111, 110, 115,
        116, 114, 117, 99, 116, 111, 114, 0, 1, 3, 114, 117, 110, 0, 2, 10, 21, 2, 2, 0,
        11, 16, 0, 65, 0, 65, 3, 65, 3, 65, 3, 65, 6, 65, 4, 16, 0, 11, 11, 26, 3, 0, 65,
        0, 11, 3, 116, 119, 111, 0, 65, 3, 11, 3, 114, 117, 110, 0, 65, 6, 11, 4, 1, 2, 3,
        4, 0, 60, 4, 110, 97, 109, 101, 1, 12, 2, 0, 4, 99, 97, 108, 108, 2, 3, 114, 117,
        110, 4, 9, 2, 0, 2, 116, 48, 1, 2, 116, 49, 6, 13, 1, 0, 10, 101, 110, 118, 46, 109,
        101, 109, 111, 114, 121, 9, 13, 3, 0, 2, 100, 48, 1, 2, 100, 49, 2, 2, 100, 50
      )

      val wrongBytecode = Array[Byte](
        0, 14, 21, 1, 2
      )

      executor.validateBytecode(bytecode) shouldBe 0
      executor.validateBytecode(wrongBytecode) shouldBe 100
    }
  }
}
