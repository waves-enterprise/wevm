package com.wavesenterprise.wasm.core

import com.google.common.io.{ByteArrayDataOutput, ByteStreams}
import com.wavesenterprise.state.{BooleanDataEntry, IntegerDataEntry, StringDataEntry}
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class MemorySpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  "binary_equals" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/memory.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "binary_equals", Array[Byte](), fuelLimit, service) shouldBe 0
  }

  "string_equals" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/memory.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "string_equals", Array[Byte](), fuelLimit, service) shouldBe 0
  }

  "contains" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/memory.wasm").readAllBytes()

    val string    = StringDataEntry("string", "hello, world")
    val substring = StringDataEntry("substring", "world")

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(string, substring), params)

    executor.runContract(contractId, bytecode, "contains", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = BooleanDataEntry("result", true)
    service.storage(service.contract)("result") shouldBe result
  }

  "drop" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/memory.wasm").readAllBytes()

    val string = StringDataEntry("string", "hello, world")
    val n      = IntegerDataEntry("n", 7)

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(string, n), params)

    executor.runContract(contractId, bytecode, "drop", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = StringDataEntry("result", "world")
    service.storage(service.contract)("result") shouldBe result
  }

  "drop_right" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/memory.wasm").readAllBytes()

    val string = StringDataEntry("string", "hello, world")
    val n      = IntegerDataEntry("n", 7)

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(string, n), params)

    executor.runContract(contractId, bytecode, "drop_right", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = StringDataEntry("result", "hello")
    service.storage(service.contract)("result") shouldBe result
  }

  "index_of" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/memory.wasm").readAllBytes()

    val string    = StringDataEntry("string", "hello, world")
    val substring = StringDataEntry("substring", "l")

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(string, substring), params)

    executor.runContract(contractId, bytecode, "index_of", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = IntegerDataEntry("result", 2L)
    service.storage(service.contract)("result") shouldBe result
  }

  "last_index_of" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/memory.wasm").readAllBytes()

    val string    = StringDataEntry("string", "hello, world")
    val substring = StringDataEntry("substring", "l")

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(string, substring), params)

    executor.runContract(contractId, bytecode, "last_index_of", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = IntegerDataEntry("result", 10L)
    service.storage(service.contract)("result") shouldBe result
  }

  "take" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/memory.wasm").readAllBytes()

    val string = StringDataEntry("string", "hello, world")
    val n      = IntegerDataEntry("n", 5)

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(string, n), params)

    executor.runContract(contractId, bytecode, "take", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = StringDataEntry("result", "hello")
    service.storage(service.contract)("result") shouldBe result
  }

  "take_right" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/memory.wasm").readAllBytes()

    val string = StringDataEntry("string", "hello, world")
    val n      = IntegerDataEntry("n", 5)

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(string, n), params)

    executor.runContract(contractId, bytecode, "take_right", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = StringDataEntry("result", "world")
    service.storage(service.contract)("result") shouldBe result
  }
}
