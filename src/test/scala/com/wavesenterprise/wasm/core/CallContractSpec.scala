package com.wavesenterprise.wasm.core

import com.google.common.io.{ByteArrayDataOutput, ByteStreams}
import com.wavesenterprise.state.{BinaryDataEntry, BooleanDataEntry, ByteStr, IntegerDataEntry, StringDataEntry}
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class CallContractSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  "call_contract" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/call_contract.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "call_contract", Array[Byte](), fuelLimit, service) shouldBe 0

    val integer = IntegerDataEntry("integer", 42)
    val boolean = BooleanDataEntry("boolean", true)
    val binary  = BinaryDataEntry("binary", ByteStr(Array[Byte](0, 1)))
    val string  = StringDataEntry("string", "test")

    service.storage(service.contractStorage)("integer") shouldBe integer
    service.storage(service.contractStorage)("boolean") shouldBe boolean
    service.storage(service.contractStorage)("binary") shouldBe binary
    service.storage(service.contractStorage)("string") shouldBe string

    service.payments(service.paymentIdStorage).apply(0) shouldBe ("null", 4200000000L)
    service.payments(service.paymentIdStorage).apply(1) shouldBe (service.asset, 2400000000L)

    service.balances("null")(service.contract) shouldBe 5800000000L
    service.balances(service.asset)(service.contract) shouldBe 7600000000L
  }

  "call_contract_params" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/call_contract.wasm").readAllBytes()

    val integer = IntegerDataEntry("integer", 42)
    val boolean = BooleanDataEntry("boolean", true)
    val binary  = BinaryDataEntry("binary", ByteStr(Array[Byte](0, 1)))
    val string  = StringDataEntry("string", "test")

    var data: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(integer, boolean, binary, string), data)

    val bytes = BinaryDataEntry("bytes", ByteStr(data.toByteArray()))

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(bytes), params)

    executor.runContract(contractId, bytecode, "call_contract_params", params.toByteArray(), fuelLimit, service) shouldBe 0

    service.storage(service.contractStorage)("integer") shouldBe integer
    service.storage(service.contractStorage)("boolean") shouldBe boolean
    service.storage(service.contractStorage)("binary") shouldBe binary
    service.storage(service.contractStorage)("string") shouldBe string
  }
}
