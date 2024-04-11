package com.wavesenterprise.wasm.core

import com.google.common.io.{ByteArrayDataOutput, ByteStreams}
import com.wavesenterprise.state.{BinaryDataEntry, ByteStr, IntegerDataEntry, StringDataEntry}
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class TxSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  "env0_get_tx_sender" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/tx.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "env0_get_tx_sender", Array[Byte](), fuelLimit, service) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr.decodeBase58(service.txSender).get)
    service.storage(service.contract)("result") shouldBe result
  }

  "env0_get_payments" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/tx.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "env0_get_payments", Array[Byte](), fuelLimit, service) shouldBe 0

    service.storage(service.contract)("result").value shouldBe 2
  }

  "env1_get_payments" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/tx.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "env1_get_payments", Array[Byte](), fuelLimit, service) shouldBe 0

    service.storage(service.contract)("result").value shouldBe 2
  }

  "env0_get_payment_asset_id" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/tx.wasm").readAllBytes()

    val integer = IntegerDataEntry("integer", 1)

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(integer), params)

    executor.runContract(contractId, bytecode, "env0_get_payment_asset_id", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr.decodeBase58(service.asset).get)
    service.storage(service.contract)("result") shouldBe result
  }

  "env1_get_payment_asset_id" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/tx.wasm").readAllBytes()

    val integer = IntegerDataEntry("integer", 1)

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(integer), params)

    executor.runContract(contractId, bytecode, "env1_get_payment_asset_id", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr.decodeBase58(service.asset).get)
    service.storage(service.contract)("result") shouldBe result
  }

  "env0_get_payment_amount" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/tx.wasm").readAllBytes()

    val integer = IntegerDataEntry("integer", 1)

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(integer), params)

    executor.runContract(contractId, bytecode, "env0_get_payment_amount", params.toByteArray(), fuelLimit, service) shouldBe 0

    service.storage(service.contract)("result").value shouldBe 2400000000L
  }

  "env1_get_payment_amount" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/tx.wasm").readAllBytes()

    val integer = IntegerDataEntry("integer", 1)

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(integer), params)

    executor.runContract(contractId, bytecode, "env1_get_payment_amount", params.toByteArray(), fuelLimit, service) shouldBe 0

    service.storage(service.contract)("result").value shouldBe 2400000000L
  }

  "env1_tx" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/tx.wasm").readAllBytes()

    val string = StringDataEntry("string", "sender")

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(string), params)

    executor.runContract(contractId, bytecode, "env1_tx", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr.decodeBase58(service.txSender).get)
    service.storage(service.contract)("result") shouldBe result
  }
}
