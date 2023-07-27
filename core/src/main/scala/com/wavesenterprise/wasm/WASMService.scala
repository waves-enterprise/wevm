package com.wavesenterprise.wasm

trait WASMService {
  /*
  def getBytecode(contractId: ByteStr): Array[Byte]

  def getStorage(contractId: ByteStr, key: Array[Byte]): Array[Byte]

  def setStorage(contractId: ByteStr, key: Array[Byte], value: Array[Byte])

  def getBalance(assetId: ByteStr, address: ByteStr): Long

  def transfer(assetId: ByteStr, address: ByteStr, amount: Long)
   */

  def getBlockTimestamp: Long

  def getBlockHeight: Long

  def getTxSender: Array[Byte]
}
