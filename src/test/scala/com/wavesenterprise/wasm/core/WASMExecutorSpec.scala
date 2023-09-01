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
  val service = new WASMServiceMock

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

    "transfer" in {
      val contractId = Base58.decode(service.contract).get
      val bytecode = getClass.getResourceAsStream("/transfer.wasm").readAllBytes()

      executor.runContract(contractId, bytecode, "_constructor", Array[Byte](), service) shouldBe 0

      service.balances("null")(service.contract) shouldBe 9999999958L
      service.balances("null")("3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU") shouldBe 10000000042L
    }

    "storage" in {
      val contractId = Base58.decode(service.contract).get
      val bytecode = getClass.getResourceAsStream("/storage.wasm").readAllBytes()

      val entry1 = IntegerDataEntry("integer_key", 42)
      val entry2 = BooleanDataEntry("boolean_key", true)
      val entry3 = BinaryDataEntry("binary_key", ByteStr(Array[Byte](0, 1)))
      val entry4 = StringDataEntry("string_key", "test")

      var args: ByteArrayDataOutput = ByteStreams.newDataOutput()
      writeDataEntryList(List(entry1, entry2, entry3, entry4), args)

      executor.runContract(contractId, bytecode, "_constructor", args.toByteArray(), service) shouldBe 0

      service.storage("integer_key") shouldBe entry1
      service.storage("boolean_key") shouldBe entry2
      service.storage("binary_key") shouldBe entry3
      service.storage("string_key") shouldBe entry4
    }
  }
}
