package com.wavesenterprise.wasm.core

import com.google.common.io.{ByteArrayDataOutput, ByteStreams}
import com.wavesenterprise.state.StringDataEntry
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class BlockSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  "env0_get_block_timestamp" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/block.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "env0_get_block_timestamp", Array[Byte](), fuelLimit, service) shouldBe 0

    service.storage(service.contract)("result").value shouldBe 1690202857485L
  }

  "env0_get_block_height" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/block.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "env0_get_block_height", Array[Byte](), fuelLimit, service) shouldBe 0

    service.storage(service.contract)("result").value shouldBe 3745592L
  }

  "env1_block" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/block.wasm").readAllBytes()

    val field = StringDataEntry("field", "timestamp")

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(field), params)

    executor.runContract(contractId, bytecode, "env1_block", params.toByteArray(), fuelLimit, service) shouldBe 0

    service.storage(service.contract)("result").value shouldBe 1690202857485L
  }
}
