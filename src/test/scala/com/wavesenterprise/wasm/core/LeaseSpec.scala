package com.wavesenterprise.wasm.core

import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class LeaseSpec extends AnyFreeSpec with Matchers {
  val bytecode = getClass.getResourceAsStream("/lease.wasm").readAllBytes()

  "lease_address" in {
    val simulator = new Simulator(bytecode)

    simulator.callMethod("lease_address", Array.empty[Byte]) shouldBe 0
  }

  "lease_alias" in {
    val simulator = new Simulator(bytecode)

    simulator.callMethod("lease_alias", Array.empty[Byte]) shouldBe 0
  }
}
