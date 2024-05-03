package com.wavesenterprise.wasm.core

import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class WASMExecutorSpec extends AnyFreeSpec with Matchers {
  val bytecode = getClass.getResourceAsStream("/mock.wasm").readAllBytes()

  "validate bytecode" in {
    val executor = new WASMExecutor

    val wrongBytecode = Array[Byte](
      0, 14, 21, 1, 2
    )

    executor.validateBytecode(bytecode) shouldBe 0
    executor.validateBytecode(wrongBytecode) shouldBe 100
  }

  "infinite_loop" in {
    val simulator = new Simulator(bytecode)

    simulator.callMethod("infinite_loop", Array.empty[Byte]) shouldBe 111
  }

  "recursion" in {
    val simulator = new Simulator(bytecode)

    simulator.callMethod("recursion", Array.empty[Byte]) shouldBe 111
  }
}
