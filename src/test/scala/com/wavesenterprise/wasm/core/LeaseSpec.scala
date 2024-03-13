package com.wavesenterprise.wasm.core

import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class LeaseSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  "lease_address" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/lease.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "lease_address", Array[Byte](), fuelLimit, service) shouldBe 0
  }

  "lease_alias" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/lease.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "lease_alias", Array[Byte](), fuelLimit, service) shouldBe 0
  }
}
