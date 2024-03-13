package com.wavesenterprise.wasm.core

import com.google.common.io.{ByteArrayDataOutput, ByteStreams}
import com.wavesenterprise.crypto.internals.WavesAlgorithms
import com.wavesenterprise.state.{BinaryDataEntry, BooleanDataEntry, ByteStr}
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class CryptoSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  "fast_hash" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/crypto.wasm").readAllBytes()

    val bytes  = Array[Byte](0, 0, 0, 1)
    val binary = BinaryDataEntry("bytes", ByteStr(bytes))

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(binary), params)

    executor.runContract(contractId, bytecode, "fast_hash", params.toByteArray(), fuelLimit, service) shouldBe 0

    val hash   = WavesAlgorithms.fastHash(bytes)
    val result = BinaryDataEntry("result", ByteStr(hash))
    service.storage(service.contract)("result") shouldBe result
  }

  "secure_hash" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/crypto.wasm").readAllBytes()

    val bytes  = Array[Byte](0, 0, 0, 1)
    val binary = BinaryDataEntry("bytes", ByteStr(bytes))

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(binary), params)

    executor.runContract(contractId, bytecode, "secure_hash", params.toByteArray(), fuelLimit, service) shouldBe 0

    val hash   = WavesAlgorithms.secureHash(bytes)
    val result = BinaryDataEntry("result", ByteStr(hash))
    service.storage(service.contract)("result") shouldBe result
  }

  "sig_verify" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/crypto.wasm").readAllBytes()

    val message      = Array[Byte](0, 0, 0, 1)
    val messageEntry = BinaryDataEntry("message", ByteStr(message))

    val keyPair = WavesAlgorithms.generateKeyPair

    val signature      = WavesAlgorithms.sign(keyPair.getPrivate, message)
    val signatureEntry = BinaryDataEntry("signature", ByteStr(signature))

    val publicKeyEntry = BinaryDataEntry("public_key", ByteStr(keyPair.getPublic.getEncoded))

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(messageEntry, signatureEntry, publicKeyEntry), params)

    executor.runContract(contractId, bytecode, "sig_verify", params.toByteArray(), fuelLimit, service) shouldBe 0

    val result = BooleanDataEntry("result", true)
    service.storage(service.contract)("result") shouldBe result
  }
}
