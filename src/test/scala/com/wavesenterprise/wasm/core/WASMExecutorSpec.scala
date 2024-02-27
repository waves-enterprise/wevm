package com.wavesenterprise.wasm.core

import com.google.common.io.{ByteArrayDataOutput, ByteStreams}
import com.wavesenterprise.state.{ByteStr, DataEntry, IntegerDataEntry, BooleanDataEntry, BinaryDataEntry, StringDataEntry}
import com.wavesenterprise.serialization.BinarySerializer
import com.wavesenterprise.transaction.docker.ContractTransactionEntryOps.toBytes
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class WASMExecutorSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  def writeDataEntryList(dataEntryList: List[DataEntry[_]], output: ByteArrayDataOutput): Unit =
    BinarySerializer.writeShortIterable(dataEntryList, dataEntryWrite, output)

  def dataEntryWrite(value: DataEntry[_], output: ByteArrayDataOutput): Unit =
    output.write(toBytes(value))

  "WASMExecutor" - {
    "validate bytecode" in {
      val bytecode = getClass.getResourceAsStream("/mock.wasm").readAllBytes()

      val wrongBytecode = Array[Byte](
        0, 14, 21, 1, 2
      )

      executor.validateBytecode(bytecode) shouldBe 0
      executor.validateBytecode(wrongBytecode) shouldBe 100
    }

    "storage" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/storage.wasm").readAllBytes()

      val entry1 = IntegerDataEntry("integer_key", 42)
      val entry2 = BooleanDataEntry("boolean_key", true)
      val entry3 = BinaryDataEntry("binary_key", ByteStr(Array[Byte](0, 1)))
      val entry4 = StringDataEntry("string_key", "test")

      var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
      writeDataEntryList(List(entry1, entry2, entry3, entry4), params)

      executor.runContract(contractId, bytecode, "save", params.toByteArray(), service) shouldBe 0

      service.storage(service.contract)("integer_key") shouldBe entry1
      service.storage(service.contract)("boolean_key") shouldBe entry2
      service.storage(service.contract)("binary_key") shouldBe entry3
      service.storage(service.contract)("string_key") shouldBe entry4

      val caller = BinaryDataEntry("caller", ByteStr(Array.empty[Byte]))
      service.storage(service.contract)("caller") shouldBe caller
    }

    "transfer" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/transfer.wasm").readAllBytes()

      executor.runContract(contractId, bytecode, "_constructor", Array[Byte](), service) shouldBe 0

      service.balances("null")(service.contract) shouldBe 9999999958L
      service.balances("null")("3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU") shouldBe 10000000042L
    }

    "asset" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/asset.wasm").readAllBytes()

      executor.runContract(contractId, bytecode, "_constructor", Array[Byte](), service) shouldBe 0

      service.balances(service.asset)(service.contract) shouldBe 9999999982L
    }

    "check balance" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/asset.wasm").readAllBytes()

      val address = BinaryDataEntry("address", ByteStr(Base58.decode(service.txSender).get))

      var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
      writeDataEntryList(List(address), params)

      executor.runContract(contractId, bytecode, "check_balance", params.toByteArray(), service) shouldBe 0

      service.storage(service.contract)("balance").value shouldBe service.balances("null")(service.txSender)
    }

    "lease" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/lease.wasm").readAllBytes()

      executor.runContract(contractId, bytecode, "_constructor", Array[Byte](), service) shouldBe 0
    }

    "block and tx" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/block_and_tx.wasm").readAllBytes()

      executor.runContract(contractId, bytecode, "_constructor", Array[Byte](), service) shouldBe 0

      val txSender         = BinaryDataEntry("tx_sender", ByteStr.decodeBase58(service.txSender).get)
      val txPaymentAssetId = BinaryDataEntry("tx_payment_asset_id", ByteStr.decodeBase58(service.asset).get)

      service.storage(service.contract)("block_timestamp").value shouldBe 1690202857485L
      service.storage(service.contract)("block_height").value shouldBe 3745592L
      service.storage(service.contract)("tx_sender") shouldBe txSender
      service.storage(service.contract)("tx_payments").value shouldBe 2
      service.storage(service.contract)("tx_payment_asset_id") shouldBe txPaymentAssetId
      service.storage(service.contract)("tx_payment_amount").value shouldBe 2400000000L

      val txSenderField = BinaryDataEntry("tx_sender_field", ByteStr.decodeBase58(service.txSender).get)
      service.storage(service.contract)("tx_sender_field") shouldBe txSenderField
    }

    "call contract" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/call_contract.wasm").readAllBytes()

      executor.runContract(contractId, bytecode, "_constructor", Array[Byte](), service) shouldBe 0

      val entry1 = IntegerDataEntry("integer_key", 42)
      val entry2 = BooleanDataEntry("boolean_key", true)
      val entry3 = BinaryDataEntry("binary_key", ByteStr(Array[Byte](0, 1)))
      val entry4 = StringDataEntry("string_key", "test")

      service.storage(service.contractMock)("integer_key") shouldBe entry1
      service.storage(service.contractMock)("boolean_key") shouldBe entry2
      service.storage(service.contractMock)("binary_key") shouldBe entry3
      service.storage(service.contractMock)("string_key") shouldBe entry4

      val caller = BinaryDataEntry("caller", ByteStr.decodeBase58(service.contract).get)
      service.storage(service.contractMock)("caller") shouldBe caller

      service.payments(service.paymentIdMock).apply(0) shouldBe ("null", 4200000000L)
      service.payments(service.paymentIdMock).apply(1) shouldBe (service.asset, 2400000000L)

      service.balances("null")(service.contract) shouldBe 5800000000L
      service.balances(service.asset)(service.contract) shouldBe 7600000000L
    }

    "call contract params" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/call_contract.wasm").readAllBytes()

      val entry1 = IntegerDataEntry("integer_key", 42)
      val entry2 = BooleanDataEntry("boolean_key", true)
      val entry3 = BinaryDataEntry("binary_key", ByteStr(Array[Byte](0, 1)))
      val entry4 = StringDataEntry("string_key", "test")

      var args: ByteArrayDataOutput = ByteStreams.newDataOutput()
      writeDataEntryList(List(entry1, entry2, entry3, entry4), args)

      val binaryParam = BinaryDataEntry("binary_param", ByteStr(args.toByteArray()))

      var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
      writeDataEntryList(List(binaryParam), params)

      executor.runContract(contractId, bytecode, "call_contract_params", params.toByteArray(), service) shouldBe 0

      service.storage(service.contractMock)("integer_key") shouldBe entry1
      service.storage(service.contractMock)("boolean_key") shouldBe entry2
      service.storage(service.contractMock)("binary_key") shouldBe entry3
      service.storage(service.contractMock)("string_key") shouldBe entry4
    }

    "base58" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/base58.wasm").readAllBytes()

      val entry = StringDataEntry("address", "3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU")

      var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
      writeDataEntryList(List(entry), params)

      executor.runContract(contractId, bytecode, "_constructor", params.toByteArray(), service) shouldBe 0

      service.storage(service.contract)("address") shouldBe entry
    }

    "utils" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/utils.wasm").readAllBytes()

      executor.runContract(contractId, bytecode, "_constructor", Array[Byte](), service) shouldBe 0
    }

    "to_le_bytes" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/utils.wasm").readAllBytes()

      val binary = BinaryDataEntry("integer", ByteStr(Array[Byte](0, 0, 0, 0, 0, 0, 0, 42)))

      var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
      writeDataEntryList(List(binary), params)

      executor.runContract(contractId, bytecode, "to_le_bytes", params.toByteArray(), service) shouldBe 0

      val integer = IntegerDataEntry("integer", 42)
      service.storage(service.contract)("integer") shouldBe integer
    }
  }
}
