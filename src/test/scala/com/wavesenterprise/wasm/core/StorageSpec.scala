package com.wavesenterprise.wasm.core

import com.wavesenterprise.state.{BinaryDataEntry, BooleanDataEntry, ByteStr, IntegerDataEntry, StringDataEntry}
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

import java.nio.charset.StandardCharsets.UTF_8

class StorageSpec extends AnyFreeSpec with Matchers {
  val bytecode  = getClass.getResourceAsStream("/storage.wasm").readAllBytes()
  val simulator = new Simulator(bytecode)

  "set_storage" in {
    val integer = IntegerDataEntry("integer", 42)
    val boolean = BooleanDataEntry("boolean", true)
    val binary  = BinaryDataEntry("binary", ByteStr(Array[Byte](0, 1)))
    val string  = StringDataEntry("string", "test")
    val params  = serializeDataEntryList(List(integer, boolean, binary, string))

    simulator.callMethod("set_storage", params) shouldBe 0

    parseDataEntry(simulator.getStorage("integer".getBytes(UTF_8))) shouldBe integer
    parseDataEntry(simulator.getStorage("boolean".getBytes(UTF_8))) shouldBe boolean
    parseDataEntry(simulator.getStorage("binary".getBytes(UTF_8))) shouldBe binary
    parseDataEntry(simulator.getStorage("string".getBytes(UTF_8))) shouldBe string
  }

  "contains_key" in {
    simulator.callMethod("contains_key", Array.empty[Byte]) shouldBe 0

    val result = BooleanDataEntry("result", true)
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "get_storage_int" in {
    val key    = StringDataEntry("key", "integer")
    val params = serializeDataEntryList(List(key))

    simulator.callMethod("get_storage_int", params) shouldBe 0

    val result = IntegerDataEntry("result", 42)
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "get_storage_bool" in {
    val key    = StringDataEntry("key", "boolean")
    val params = serializeDataEntryList(List(key))

    simulator.callMethod("get_storage_bool", params) shouldBe 0

    val result = BooleanDataEntry("result", true)
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "get_storage_binary" in {
    val key    = StringDataEntry("key", "binary")
    val params = serializeDataEntryList(List(key))

    simulator.callMethod("get_storage_binary", params) shouldBe 0

    val result = BinaryDataEntry("result", ByteStr(Array[Byte](0, 1)))
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "get_storage_string" in {
    val key    = StringDataEntry("key", "string")
    val params = serializeDataEntryList(List(key))

    simulator.callMethod("get_storage_string", params) shouldBe 0

    val result = StringDataEntry("result", "test")
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }
}
