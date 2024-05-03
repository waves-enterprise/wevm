package com.wavesenterprise.wasm.core

import com.wavesenterprise.state.{BinaryDataEntry, BooleanDataEntry, ByteStr, IntegerDataEntry, StringDataEntry}
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

import java.nio.charset.StandardCharsets.UTF_8

class CallContractSpec extends AnyFreeSpec with Matchers {
  val bytecode = getClass.getResourceAsStream("/call_contract.wasm").readAllBytes()

  "call_contract" in {
    val simulator = new Simulator(bytecode)

    // Load storage contract
    val storageBytecode   = getClass.getResourceAsStream("/storage.wasm").readAllBytes()
    val storageContractId = simulator.loadAdditionalBytecode(storageBytecode)

    // Create asset
    val assetId = simulator.issue(
      "Asset".getBytes(UTF_8),
      "Super asset".getBytes(UTF_8),
      10000000000L,
      8L,
      false,
    )

    simulator.transfer(simulator.accounts(0), Array.empty[Byte], simulator.contractId, 10000000000L)
    simulator.transfer(simulator.accounts(0), assetId, simulator.contractId, 10000000000L)

    // Function parameters
    val contractId = StringDataEntry("contractId", Base58.encode(storageContractId))
    val funcName   = StringDataEntry("funcName", "set_storage")
    val asset      = StringDataEntry("asset", Base58.encode(assetId))
    val params     = serializeDataEntryList(List(contractId, funcName, asset))

    simulator.callMethod("call_contract", params) shouldBe 0

    val integer = IntegerDataEntry("integer", 42)
    val boolean = BooleanDataEntry("boolean", true)
    val binary  = BinaryDataEntry("binary", ByteStr(Array[Byte](0, 1)))
    val string  = StringDataEntry("string", "test")

    // Check if the storage contract has recorded the data sent by our contract
    parseDataEntry(simulator.getStorage(storageContractId, "integer".getBytes(UTF_8))) shouldBe integer
    parseDataEntry(simulator.getStorage(storageContractId, "boolean".getBytes(UTF_8))) shouldBe boolean
    parseDataEntry(simulator.getStorage(storageContractId, "binary".getBytes(UTF_8))) shouldBe binary
    parseDataEntry(simulator.getStorage(storageContractId, "string".getBytes(UTF_8))) shouldBe string

    // Check whether the funds enclosed by the payment have gone away
    simulator.getBalance(Array.empty[Byte]) shouldBe 5800000000L
    simulator.getBalance(assetId) shouldBe 7600000000L

    // Checking to see if the contract has received funds
    simulator.getBalance(Array.empty[Byte], storageContractId) shouldBe 4200000000L
    simulator.getBalance(assetId, storageContractId) shouldBe 2400000000L
  }

  "call_contract_params" in {
    val simulator = new Simulator(bytecode)

    // Load storage contract
    val storageBytecode   = getClass.getResourceAsStream("/storage.wasm").readAllBytes()
    val storageContractId = simulator.loadAdditionalBytecode(storageBytecode)

    val integer = IntegerDataEntry("integer", 42)
    val boolean = BooleanDataEntry("boolean", true)
    val binary  = BinaryDataEntry("binary", ByteStr(Array[Byte](0, 1)))
    val string  = StringDataEntry("string", "test")
    val data    = serializeDataEntryList(List(integer, boolean, binary, string))

    // Function parameters
    val contractId = StringDataEntry("contractId", Base58.encode(storageContractId))
    val funcName   = StringDataEntry("funcName", "set_storage")
    val bytes      = BinaryDataEntry("bytes", ByteStr(data))
    val params     = serializeDataEntryList(List(contractId, funcName, bytes))

    simulator.callMethod("call_contract_params", params) shouldBe 0

    // Check if the storage contract has recorded the data sent by our contract
    parseDataEntry(simulator.getStorage(storageContractId, "integer".getBytes(UTF_8))) shouldBe integer
    parseDataEntry(simulator.getStorage(storageContractId, "boolean".getBytes(UTF_8))) shouldBe boolean
    parseDataEntry(simulator.getStorage(storageContractId, "binary".getBytes(UTF_8))) shouldBe binary
    parseDataEntry(simulator.getStorage(storageContractId, "string".getBytes(UTF_8))) shouldBe string
  }
}
