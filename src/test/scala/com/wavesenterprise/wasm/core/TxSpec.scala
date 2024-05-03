package com.wavesenterprise.wasm.core

import com.wavesenterprise.state.{BinaryDataEntry, ByteStr, IntegerDataEntry, StringDataEntry}
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

import java.nio.charset.StandardCharsets.UTF_8

class TxSpec extends AnyFreeSpec with Matchers {
  val bytecode  = getClass.getResourceAsStream("/tx.wasm").readAllBytes()
  val simulator = new Simulator(bytecode)

  val assetId = simulator.issue(
    "Asset".getBytes(UTF_8),
    "Super asset".getBytes(UTF_8),
    10000000000L,
    8L,
    false,
  )

  simulator.addPayment(Array.empty[Byte], 4200000000L)
  simulator.addPayment(assetId, 2400000000L)

  "env0_get_tx_sender" in {
    simulator.callMethod("env0_get_tx_sender", Array.empty[Byte]) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr(simulator.accounts(0)))
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "env0_get_payments" in {
    simulator.callMethod("env0_get_payments", Array.empty[Byte]) shouldBe 0

    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))).value shouldBe 2
  }

  "env1_get_payments" in {
    simulator.callMethod("env1_get_payments", Array.empty[Byte]) shouldBe 0

    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))).value shouldBe 2
  }

  "env0_get_payment_asset_id" in {
    val integer = IntegerDataEntry("integer", 1)
    val params  = serializeDataEntryList(List(integer))

    simulator.callMethod("env0_get_payment_asset_id", params) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr(assetId))
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "env1_get_payment_asset_id" in {
    val integer = IntegerDataEntry("integer", 1)
    val params  = serializeDataEntryList(List(integer))

    simulator.callMethod("env1_get_payment_asset_id", params) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr(assetId))
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "env0_get_payment_amount" in {
    val integer = IntegerDataEntry("integer", 1)
    val params  = serializeDataEntryList(List(integer))

    simulator.callMethod("env0_get_payment_amount", params) shouldBe 0

    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))).value shouldBe 2400000000L
  }

  "env1_get_payment_amount" in {
    val integer = IntegerDataEntry("integer", 1)
    val params  = serializeDataEntryList(List(integer))

    simulator.callMethod("env1_get_payment_amount", params) shouldBe 0

    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))).value shouldBe 2400000000L
  }

  "env1_tx" in {
    val string = StringDataEntry("string", "sender")
    val params = serializeDataEntryList(List(string))

    simulator.callMethod("env1_tx", params) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr(simulator.accounts(0)))
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }
}
