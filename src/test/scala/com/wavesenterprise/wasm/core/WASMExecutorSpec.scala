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

      var args: ByteArrayDataOutput = ByteStreams.newDataOutput()
      writeDataEntryList(List(entry1, entry2, entry3, entry4), args)

      executor.runContract(contractId, bytecode, "save", args.toByteArray(), service) shouldBe 0

      service.storage(service.contract)("integer_key") shouldBe entry1
      service.storage(service.contract)("boolean_key") shouldBe entry2
      service.storage(service.contract)("binary_key") shouldBe entry3
      service.storage(service.contract)("string_key") shouldBe entry4
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

      var args: ByteArrayDataOutput = ByteStreams.newDataOutput()
      writeDataEntryList(List(address), args)

      executor.runContract(contractId, bytecode, "check_balance", args.toByteArray(), service) shouldBe 0

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

      service.payments(service.paymentIdMock).apply(0) shouldBe ("null", 4200000000L)
      service.payments(service.paymentIdMock).apply(1) shouldBe (service.asset, 2400000000L)

      service.balances("null")(service.contract) shouldBe 5800000000L
      service.balances(service.asset)(service.contract) shouldBe 7600000000L
    }

    "base58" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/base58.wasm").readAllBytes()

      val entry = StringDataEntry("address", "3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU")

      var args: ByteArrayDataOutput = ByteStreams.newDataOutput()
      writeDataEntryList(List(entry), args)

      executor.runContract(contractId, bytecode, "_constructor", args.toByteArray(), service) shouldBe 0

      service.storage(service.contract)("address") shouldBe entry
    }

    "utils" in {
      val service = new WASMServiceMock

      val contractId = Base58.decode(service.contract).get
      val bytecode   = getClass.getResourceAsStream("/utils.wasm").readAllBytes()

      executor.runContract(contractId, bytecode, "_constructor", Array[Byte](), service) shouldBe 0
    }
  }
}
