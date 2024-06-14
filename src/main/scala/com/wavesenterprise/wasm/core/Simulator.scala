package com.wavesenterprise.wasm.core

import com.wavesenterprise.account.Address
import com.wavesenterprise.crypto.internals.WavesAlgorithms

import java.nio.ByteBuffer

import scala.annotation.tailrec

class Simulator(bytecode: Array[Byte]) {
  private val executor = new WASMExecutor
  private val service  = new SimulatorWASMService

  val contractId: Array[Byte] = WavesAlgorithms.secureHash(bytecode)

  private var _fuelLimit: Long = 1024L

  def fuelLimit: Long           = this._fuelLimit
  def setFuelLimit(value: Long) = this._fuelLimit = value

  def chainId: Byte           = this.service.getChainId()
  def setChainId(value: Byte) = this.service.setChainId(value)

  def errorMessage: String = this.service.errorMessage

  def timestamp: Long           = this.service.timestamp
  def setTimestamp(value: Long) = this.service.setTimestamp(value)

  def height: Long = this.service.height

  private val _accounts: Array[Array[Byte]] = generateAccounts(Array.empty[Array[Byte]])

  def accounts(i: Int): Array[Byte] =
    this._accounts(i)

  @tailrec
  private def generateAccounts(accounts: Array[Array[Byte]]): Array[Array[Byte]] = {
    val keyPair = WavesAlgorithms.generateKeyPair
    val address = Address.fromPublicKey(keyPair.getPublic.getEncoded, service.getChainId, WavesAlgorithms).bytes.arr

    val wSystemToken = ByteBuffer.wrap(Array.empty[Byte])
    val wAddress     = ByteBuffer.wrap(address)

    this.service.updateBalance(wSystemToken, wAddress, 10000000000L)

    if (accounts.length < 5) generateAccounts(accounts :+ address) else accounts
  }

  def loadAdditionalBytecode(bytecode: Array[Byte]): Array[Byte] = {
    val contractId: Array[Byte] = WavesAlgorithms.secureHash(bytecode)
    val wContractId             = ByteBuffer.wrap(contractId)
    this.service.setBytecode(wContractId, bytecode)
    contractId
  }

  def callMethod(funcName: String, params: Array[Byte]): Int = {
    this.service.setTxSender(this._accounts(0))
    this.executor.runContract(this.contractId, this.bytecode, funcName, params, this.fuelLimit, this.service)
  }

  def getBalance(assetId: Array[Byte], assetHolder: Array[Byte]): Long =
    this.service.getBalance(ByteBuffer.wrap(assetId), ByteBuffer.wrap(assetHolder))

  def getBalance(assetId: Array[Byte]): Long =
    this.service.getBalance(ByteBuffer.wrap(assetId), ByteBuffer.wrap(this.contractId))

  def transfer(sender: Array[Byte], assetId: Array[Byte], recipient: Array[Byte], amount: Long) =
    this.service.transfer(ByteBuffer.wrap(sender), ByteBuffer.wrap(assetId), ByteBuffer.wrap(recipient), amount)

  def issue(name: Array[Byte], description: Array[Byte], quantity: Long, decimals: Long, isReissuable: Boolean): Array[Byte] =
    this.service.issue(this._accounts(0), name, description, quantity, decimals, isReissuable)

  def getStorage(contractId: Array[Byte], key: Array[Byte]): Array[Byte] =
    this.service.getStorage(contractId, key)

  def getStorage(key: Array[Byte]): Array[Byte] =
    this.service.getStorage(this.contractId, key)

  def addPayment(assetId: Array[Byte], amount: Long) = {
    this.service.addPayment(this.contractId, assetId, amount)
    this.service.transfer(ByteBuffer.wrap(this._accounts(0)), ByteBuffer.wrap(assetId), ByteBuffer.wrap(this.contractId), amount)
  }
}
