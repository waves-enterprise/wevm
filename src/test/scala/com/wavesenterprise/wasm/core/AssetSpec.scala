package com.wavesenterprise.wasm.core

import com.wavesenterprise.state.{BinaryDataEntry, ByteStr}
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

import java.nio.charset.StandardCharsets.UTF_8

class AssetSpec extends AnyFreeSpec with Matchers {
  val bytecode = getClass.getResourceAsStream("/asset.wasm").readAllBytes()

  "env0_asset" in {
    val simulator = new Simulator(bytecode)

    simulator.callMethod("env0_asset", Array.empty[Byte]) shouldBe 0

    val assetId = parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))).value match {
      case bytes: ByteStr => bytes.arr
      case _              => throw new Exception
    }

    simulator.getBalance(assetId) shouldBe 82L
  }

  "env1_asset" in {
    val simulator = new Simulator(bytecode)

    simulator.callMethod("env1_asset", Array.empty[Byte]) shouldBe 0

    val assetId = parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))).value match {
      case bytes: ByteStr => bytes.arr
      case _              => throw new Exception
    }

    simulator.getBalance(assetId) shouldBe 82L
  }

  "env0_get_balance" in {
    val simulator = new Simulator(bytecode)

    val address = BinaryDataEntry("address", ByteStr(simulator.accounts(0)))
    val params  = serializeDataEntryList(List(address))

    simulator.callMethod("env0_get_balance", params) shouldBe 0

    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))).value shouldBe 10000000000L
  }

  "env1_get_balance" in {
    val simulator = new Simulator(bytecode)

    val address = BinaryDataEntry("address", ByteStr(simulator.accounts(0)))
    val params  = serializeDataEntryList(List(address))

    simulator.callMethod("env1_get_balance", params) shouldBe 0

    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))).value shouldBe 10000000000L
  }

  "env0_transfer" in {
    val simulator = new Simulator(bytecode)

    simulator.transfer(simulator.accounts(0), Array.empty[Byte], simulator.contractId, 5000000000L)

    val address = BinaryDataEntry("address", ByteStr(simulator.accounts(0)))
    val params  = serializeDataEntryList(List(address))

    simulator.callMethod("env0_transfer", params) shouldBe 0

    simulator.getBalance(Array.empty[Byte]) shouldBe 4999999958L
    simulator.getBalance(Array.empty[Byte], simulator.accounts(0)) shouldBe 5000000042L
  }

  "env1_transfer" in {
    val simulator = new Simulator(bytecode)

    simulator.transfer(simulator.accounts(0), Array.empty[Byte], simulator.contractId, 5000000000L)

    val address = BinaryDataEntry("address", ByteStr(simulator.accounts(0)))
    val params  = serializeDataEntryList(List(address))

    simulator.callMethod("env1_transfer", params) shouldBe 0

    simulator.getBalance(Array.empty[Byte]) shouldBe 4999999958L
    simulator.getBalance(Array.empty[Byte], simulator.accounts(0)) shouldBe 5000000042L
  }
}
