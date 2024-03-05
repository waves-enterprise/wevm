package com.wavesenterprise.wasm.core

import com.google.common.io.{ByteArrayDataOutput, ByteStreams}
import com.wavesenterprise.state.{BinaryDataEntry, ByteStr}
import com.wavesenterprise.utils.Base58
import org.scalatest.freespec.AnyFreeSpec
import org.scalatest.matchers.should.Matchers

class AssetSpec extends AnyFreeSpec with Matchers {
  val executor = new WASMExecutor

  "asset" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/asset.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "asset", Array[Byte](), service) shouldBe 0

    service.balances(service.asset)(service.contract) shouldBe 9999999982L
  }

  "get_balance" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/asset.wasm").readAllBytes()

    val address = BinaryDataEntry("address", ByteStr(Base58.decode(service.txSender).get))

    var params: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(List(address), params)

    executor.runContract(contractId, bytecode, "get_balance", params.toByteArray(), service) shouldBe 0

    service.storage(service.contract)("result").value shouldBe service.balances("null")(service.txSender)
  }

  "transfer" in {
    val service = new WASMServiceMock

    val contractId = Base58.decode(service.contract).get
    val bytecode   = getClass.getResourceAsStream("/asset.wasm").readAllBytes()

    executor.runContract(contractId, bytecode, "transfer", Array[Byte](), service) shouldBe 0

    service.balances("null")(service.contract) shouldBe 9999999958L
    service.balances("null")("3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU") shouldBe 10000000042L
  }
}
