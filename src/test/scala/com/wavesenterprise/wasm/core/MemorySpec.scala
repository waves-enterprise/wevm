package com.wavesenterprise.wasm.core

import com.wavesenterprise.state.{BooleanDataEntry, IntegerDataEntry, StringDataEntry}
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

import java.nio.charset.StandardCharsets.UTF_8

class MemorySpec extends AnyFreeSpec with Matchers {
  val bytecode = getClass.getResourceAsStream("/memory.wasm").readAllBytes()

  "binary_equals" in {
    val simulator = new Simulator(bytecode)

    simulator.callMethod("binary_equals", Array.empty[Byte]) shouldBe 0
  }

  "string_equals" in {
    val simulator = new Simulator(bytecode)

    simulator.callMethod("string_equals", Array.empty[Byte]) shouldBe 0
  }

  "contains" in {
    val bytecode  = getClass.getResourceAsStream("/memory.wasm").readAllBytes()
    val simulator = new Simulator(bytecode)

    val string    = StringDataEntry("string", "hello, world")
    val substring = StringDataEntry("substring", "world")
    val params    = serializeDataEntryList(List(string, substring))

    simulator.callMethod("contains", params) shouldBe 0

    val result = BooleanDataEntry("result", true)
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "drop" in {
    val simulator = new Simulator(bytecode)

    val string = StringDataEntry("string", "hello, world")
    val n      = IntegerDataEntry("n", 7)
    val params = serializeDataEntryList(List(string, n))

    simulator.callMethod("drop", params) shouldBe 0

    val result = StringDataEntry("result", "world")
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "drop_right" in {
    val simulator = new Simulator(bytecode)

    val string = StringDataEntry("string", "hello, world")
    val n      = IntegerDataEntry("n", 7)
    val params = serializeDataEntryList(List(string, n))

    simulator.callMethod("drop_right", params) shouldBe 0

    val result = StringDataEntry("result", "hello")
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "index_of" in {
    val simulator = new Simulator(bytecode)

    val string    = StringDataEntry("string", "hello, world")
    val substring = StringDataEntry("substring", "l")
    val params    = serializeDataEntryList(List(string, substring))

    simulator.callMethod("index_of", params) shouldBe 0

    val result = IntegerDataEntry("result", 2L)
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "last_index_of" in {
    val simulator = new Simulator(bytecode)

    val string    = StringDataEntry("string", "hello, world")
    val substring = StringDataEntry("substring", "l")
    val params    = serializeDataEntryList(List(string, substring))

    simulator.callMethod("last_index_of", params) shouldBe 0

    val result = IntegerDataEntry("result", 10L)
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "take" in {
    val simulator = new Simulator(bytecode)

    val string = StringDataEntry("string", "hello, world")
    val n      = IntegerDataEntry("n", 5)
    val params = serializeDataEntryList(List(string, n))

    simulator.callMethod("take", params) shouldBe 0

    val result = StringDataEntry("result", "hello")
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "take_right" in {
    val simulator = new Simulator(bytecode)

    val string = StringDataEntry("string", "hello, world")
    val n      = IntegerDataEntry("n", 5)
    val params = serializeDataEntryList(List(string, n))

    simulator.callMethod("take_right", params) shouldBe 0

    val result = StringDataEntry("result", "world")
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }
}
