package com.wavesenterprise.wasm.core

import com.google.common.io.{ByteArrayDataOutput, ByteStreams}
import com.wavesenterprise.state.{BinaryDataEntry, ByteStr, IntegerDataEntry, StringDataEntry}
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class UtilsSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  "base58" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/utils.wasm").readAllBytes()

    val address = StringDataEntry("address", "3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU")

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(address), params)

    executor.runContract(contractId, bytecode, "base58", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = StringDataEntry("result", "3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU")
    service.storage(service.contract)("result") shouldBe result
  }

  "to_le_bytes" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/utils.wasm").readAllBytes()

    val bytes = BinaryDataEntry("bytes", ByteStr(Array[Byte](0, 0, 0, 0, 0, 0, 0, 42)))

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(bytes), params)

    executor.runContract(contractId, bytecode, "to_le_bytes", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = IntegerDataEntry("result", 42)
    service.storage(service.contract)("result") shouldBe result
  }

  "caller" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/utils.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "caller", Array[Byte](), fuelLimit, service) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr.decodeBase58(service.contract).get)
    service.storage(service.contractMock)("result") shouldBe result
  }
}
