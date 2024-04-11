package com.wavesenterprise.wasm.core

import com.google.common.io.{ByteArrayDataOutput, ByteStreams}
import com.wavesenterprise.state.{BinaryDataEntry, ByteStr}
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class AssetSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  "env0_asset" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/asset.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "env0_asset", Array[Byte](), fuelLimit, service) shouldBe 0

    service.balances(service.asset)(service.contract) shouldBe 9999999982L
  }

  "env1_asset" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/asset.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "env1_asset", Array[Byte](), fuelLimit, service) shouldBe 0

    service.balances(service.asset)(service.contract) shouldBe 9999999982L
  }

  "env0_get_balance" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/asset.wasm").readAllBytes()

    val address = BinaryDataEntry("address", ByteStr(Base58.decode(service.txSender).get))

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(address), params)

    executor.runContract(contractId, bytecode, "env0_get_balance", params.toByteArray(), fuelLimit, service) shouldBe 0

    service.storage(service.contract)("result").value shouldBe service.balances("null")(service.txSender)
  }

  "env1_get_balance" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/asset.wasm").readAllBytes()

    val address = BinaryDataEntry("address", ByteStr(Base58.decode(service.txSender).get))

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(address), params)

    executor.runContract(contractId, bytecode, "env1_get_balance", params.toByteArray(), fuelLimit, service) shouldBe 0

    service.storage(service.contract)("result").value shouldBe service.balances("null")(service.txSender)
  }

  "env0_transfer" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/asset.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "env0_transfer", Array[Byte](), fuelLimit, service) shouldBe 0

    service.balances("null")(service.contract) shouldBe 9999999958L
    service.balances("null")("3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU") shouldBe 10000000042L
  }

  "env1_transfer" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/asset.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "env1_transfer", Array[Byte](), fuelLimit, service) shouldBe 0

    service.balances("null")(service.contract) shouldBe 9999999958L
    service.balances("null")("3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU") shouldBe 10000000042L
  }
}
