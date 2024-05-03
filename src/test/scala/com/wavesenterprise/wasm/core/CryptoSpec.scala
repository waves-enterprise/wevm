package com.wavesenterprise.wasm.core

import com.wavesenterprise.crypto.internals.WavesAlgorithms
import com.wavesenterprise.state.{BinaryDataEntry, BooleanDataEntry, ByteStr}
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

import java.nio.charset.StandardCharsets.UTF_8

class CryptoSpec extends AnyFreeSpec with Matchers {
  val bytecode = getClass.getResourceAsStream("/crypto.wasm").readAllBytes()

  "fast_hash" in {
    val simulator = new Simulator(bytecode)

    val bytes  = Array[Byte](0, 0, 0, 1)
    val binary = BinaryDataEntry("bytes", ByteStr(bytes))
    val params = serializeDataEntryList(List(binary))

    simulator.callMethod("fast_hash", params) shouldBe 0

    val hash   = WavesAlgorithms.fastHash(bytes)
    val result = BinaryDataEntry("result", ByteStr(hash))
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "secure_hash" in {
    val simulator = new Simulator(bytecode)

    val bytes  = Array[Byte](0, 0, 0, 1)
    val binary = BinaryDataEntry("bytes", ByteStr(bytes))
    val params = serializeDataEntryList(List(binary))

    simulator.callMethod("secure_hash", params) shouldBe 0

    val hash   = WavesAlgorithms.secureHash(bytes)
    val result = BinaryDataEntry("result", ByteStr(hash))
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }

  "sig_verify" in {
    val simulator = new Simulator(bytecode)

    val message      = Array[Byte](0, 0, 0, 1)
    val messageEntry = BinaryDataEntry("message", ByteStr(message))

    val keyPair = WavesAlgorithms.generateKeyPair

    val signature      = WavesAlgorithms.sign(keyPair.getPrivate, message)
    val signatureEntry = BinaryDataEntry("signature", ByteStr(signature))

    val publicKeyEntry = BinaryDataEntry("public_key", ByteStr(keyPair.getPublic.getEncoded))

    val params = serializeDataEntryList(List(messageEntry, signatureEntry, publicKeyEntry))

    simulator.callMethod("sig_verify", params) shouldBe 0

    val result = BooleanDataEntry("result", true)
    parseDataEntry(simulator.getStorage("result".getBytes(UTF_8))) shouldBe result
  }
}
