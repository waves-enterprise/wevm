package com.wavesenterprise.wasm.core

import com.wavesenterprise.state.StringDataEntry
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

import java.nio.charset.StandardCharsets.UTF_8

class BlockSpec extends AnyFreeSpec with Matchers {
  val bytecode = getClass.getResourceAsStream("/block.wasm").readAllBytes()

  "env0_get_block_timestamp" in {
    val simulator = new Simulator(bytecode)

    simulator.callMethod("env0_get_block_timestamp", Array.empty[Byte]) shouldBe 0

    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))).value shouldBe simulator.timestamp
  }

  "env0_get_block_height" in {
    val simulator = new Simulator(bytecode)

    simulator.callMethod("env0_get_block_height", Array.empty[Byte]) shouldBe 0

    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))).value shouldBe simulator.height
  }

  "env1_block" in {
    val simulator = new Simulator(bytecode)

    val field  = StringDataEntry("field", "timestamp")
    val params = serializeDataEntryList(List(field))

    simulator.callMethod("env1_block", params) shouldBe 0

    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))).value shouldBe simulator.timestamp
  }
}
