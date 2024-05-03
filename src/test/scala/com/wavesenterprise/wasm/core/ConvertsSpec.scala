package com.wavesenterprise.wasm.core

import com.wavesenterprise.state.{BinaryDataEntry, BooleanDataEntry, ByteStr, IntegerDataEntry, StringDataEntry}
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

import java.nio.charset.StandardCharsets.UTF_8

class ConvertsSpec extends AnyFreeSpec with Matchers {
  val bytecode = getClass.getResourceAsStream("/converts.wasm").readAllBytes()

  "parse_int" in {
    val simulator = new Simulator(bytecode)

    val integer = StringDataEntry("integer", "31337")
    val params  = serializeDataEntryList(List(integer))

    simulator.callMethod("parse_int", params) shouldBe 0

    val result = IntegerDataEntry("result", 31337L)
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "parse_bool" in {
    val simulator = new Simulator(bytecode)

    val boolean = StringDataEntry("boolean", "false")
    val params  = serializeDataEntryList(List(boolean))

    simulator.callMethod("parse_bool", params) shouldBe 0

    val result = BooleanDataEntry("result", false)
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "to_bytes" in {
    val simulator = new Simulator(bytecode)

    val integer = IntegerDataEntry("integer", 31337L)
    val params  = serializeDataEntryList(List(integer))

    simulator.callMethod("to_bytes", params) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr(Array[Byte](0, 0, 0, 0, 0, 0, 122, 105)))
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "to_int" in {
    val simulator = new Simulator(bytecode)

    val binary = BinaryDataEntry("binary", ByteStr(Array[Byte](0, 0, 0, 0, 0, 0, 122, 105)))
    val params = serializeDataEntryList(List(binary))

    simulator.callMethod("to_int", params) shouldBe 0

    val result = IntegerDataEntry("result", 31337L)
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "to_string_bool" in {
    val simulator = new Simulator(bytecode)

    val boolean = BooleanDataEntry("boolean", false)
    val params  = serializeDataEntryList(List(boolean))

    simulator.callMethod("to_string_bool", params) shouldBe 0

    val result = StringDataEntry("result", "false")
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "to_string_int" in {
    val simulator = new Simulator(bytecode)

    val integer = IntegerDataEntry("integer", 31337L)
    val params  = serializeDataEntryList(List(integer))

    simulator.callMethod("to_string_int", params) shouldBe 0

    val result = StringDataEntry("result", "31337")
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }
}
