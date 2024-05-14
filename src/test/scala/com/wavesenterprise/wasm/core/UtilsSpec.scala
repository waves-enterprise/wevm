package com.wavesenterprise.wasm.core

import com.wavesenterprise.state.{BinaryDataEntry, ByteStr, IntegerDataEntry, StringDataEntry}
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

import java.nio.charset.StandardCharsets.UTF_8

class UtilsSpec extends AnyFreeSpec with Matchers {
  val bytecode = getClass.getResourceAsStream("/utils.wasm").readAllBytes()

  "base58" in {
    val simulator = new Simulator(bytecode)

    val address = StringDataEntry("address", "3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU")
    val params  = serializeDataEntryList(List(address))

    simulator.callMethod("base58", params) shouldBe 0

    val result = StringDataEntry("result", "3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU")
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "to_le_bytes" in {
    val simulator = new Simulator(bytecode)

    val bytes  = BinaryDataEntry("bytes", ByteStr(Array[Byte](0, 0, 0, 0, 0, 0, 0, 42)))
    val params = serializeDataEntryList(List(bytes))

    simulator.callMethod("to_le_bytes", params) shouldBe 0

    val result = IntegerDataEntry("result", 42)
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "caller" in {
    val simulator = new Simulator(bytecode)

    val mockBytecode   = getClass.getResourceAsStream("/mock.wasm").readAllBytes()
    val mockContractId = simulator.loadAdditionalBytecode(mockBytecode)

    val contractId = StringDataEntry("contractId", Base58.encode(mockContractId))
    val funcName   = StringDataEntry("funcName", "caller")
    val params     = serializeDataEntryList(List(contractId, funcName))

    simulator.callMethod("caller", params) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr(simulator.contractId))
    parseDataEntry(simulator.getStorage(mockContractId, "result".getBytes(UTF_8))) shouldBe result
  }
}
