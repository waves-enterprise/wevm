package com.wavesenterprise.wasm.core

import com.google.common.io.{ByteArrayDataOutput, ByteStreams}
import com.wavesenterprise.state.{BinaryDataEntry, BooleanDataEntry, ByteStr, IntegerDataEntry, StringDataEntry}
import com.wavesenterprise.transaction.docker.ContractTransactionEntryOps.toBytes
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class StorageSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  "set_storage" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/storage.wasm").readAllBytes()

    val integer = IntegerDataEntry("integer", 42)
    val boolean = BooleanDataEntry("boolean", true)
    val binary  = BinaryDataEntry("binary", ByteStr(Array[Byte](0, 1)))
    val string  = StringDataEntry("string", "test")

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(integer, boolean, binary, string), params)

    executor.runContract(contractId, bytecode, "set_storage", params.toByteArray(), fuelLimit, service) shouldBe 0

    service.storage(service.contract)("integer") shouldBe integer
    service.storage(service.contract)("boolean") shouldBe boolean
    service.storage(service.contract)("binary") shouldBe binary
    service.storage(service.contract)("string") shouldBe string
  }

  "get_storage_int" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/storage.wasm").readAllBytes()

    val integer = IntegerDataEntry("integer", 42)
    service.setStorage(Base58.decode(service.contract).get, toBytes(integer))

    val key = StringDataEntry("key", "integer")

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(key), params)

    executor.runContract(contractId, bytecode, "get_storage_int", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = IntegerDataEntry("result", 42)
    service.storage(service.contract)("result") shouldBe result
  }

  "get_storage_bool" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/storage.wasm").readAllBytes()

    val boolean = BooleanDataEntry("boolean", true)
    service.setStorage(Base58.decode(service.contract).get, toBytes(boolean))

    val key = StringDataEntry("key", "boolean")

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(key), params)

    executor.runContract(contractId, bytecode, "get_storage_bool", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = BooleanDataEntry("result", true)
    service.storage(service.contract)("result") shouldBe result
  }

  "get_storage_binary" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/storage.wasm").readAllBytes()

    val binary = BinaryDataEntry("binary", ByteStr(Array[Byte](0, 1)))
    service.setStorage(Base58.decode(service.contract).get, toBytes(binary))

    val key = StringDataEntry("key", "binary")

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(key), params)

    executor.runContract(contractId, bytecode, "get_storage_binary", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr(Array[Byte](0, 1)))
    service.storage(service.contract)("result") shouldBe result
  }

  "get_storage_string" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/storage.wasm").readAllBytes()

    val string = StringDataEntry("string", "test")
    service.setStorage(Base58.decode(service.contract).get, toBytes(string))

    val key = StringDataEntry("key", "string")

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(key), params)

    executor.runContract(contractId, bytecode, "get_storage_string", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = StringDataEntry("result", "test")
    service.storage(service.contract)("result") shouldBe result
  }
}
