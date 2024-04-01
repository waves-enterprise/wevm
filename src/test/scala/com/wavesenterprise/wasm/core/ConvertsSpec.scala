package com.wavesenterprise.wasm.core

import com.google.common.io.{ByteArrayDataOutput, ByteStreams}
import com.wavesenterprise.state.{BinaryDataEntry, BooleanDataEntry, ByteStr, IntegerDataEntry, StringDataEntry}
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class ConvertsSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  "parse_int" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/converts.wasm").readAllBytes()

    val integer = StringDataEntry("integer", "31337")

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(integer), params)

    executor.runContract(contractId, bytecode, "parse_int", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = IntegerDataEntry("result", 31337L)
    service.storage(service.contract)("result") shouldBe result
  }

  "parse_bool" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/converts.wasm").readAllBytes()

    val boolean = StringDataEntry("boolean", "false")

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(boolean), params)

    executor.runContract(contractId, bytecode, "parse_bool", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = BooleanDataEntry("result", false)
    service.storage(service.contract)("result") shouldBe result
  }

  "to_bytes" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/converts.wasm").readAllBytes()

    val integer = IntegerDataEntry("integer", 31337L)

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(integer), params)

    executor.runContract(contractId, bytecode, "to_bytes", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr(Array[Byte](0, 0, 0, 0, 0, 0, 122, 105)))
    service.storage(service.contract)("result") shouldBe result
  }

  "to_int" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/converts.wasm").readAllBytes()

    val binary = BinaryDataEntry("binary", ByteStr(Array[Byte](0, 0, 0, 0, 0, 0, 122, 105)))

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(binary), params)

    executor.runContract(contractId, bytecode, "to_int", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = IntegerDataEntry("result", 31337L)
    service.storage(service.contract)("result") shouldBe result
  }

  "to_string_bool" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/converts.wasm").readAllBytes()

    val boolean = BooleanDataEntry("boolean", false)

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(boolean), params)

    executor.runContract(contractId, bytecode, "to_string_bool", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = StringDataEntry("result", "false")
    service.storage(service.contract)("result") shouldBe result
  }

  "to_string_int" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/converts.wasm").readAllBytes()

    val integer = IntegerDataEntry("integer", 31337L)

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(integer), params)

    executor.runContract(contractId, bytecode, "to_string_int", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = StringDataEntry("result", "31337")
    service.storage(service.contract)("result") shouldBe result
  }
}
