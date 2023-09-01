package com.wavesenterprise.wasm.core

import com.wavesenterprise.state.DataEntry
import com.wavesenterprise.transaction.docker.ContractTransactionEntryOps.{parse, toBytes}
import com.wavesenterprise.utils.Base58

import scala.collection.mutable.Map

class WASMServiceMock extends WASMService {
  val contract = "4WVhw3QdiinpE5QXDG7QfqLiLanM7ewBw4ChX4qyGjs2"
  val txSender = "3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU"
  val recipient = "3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3"
  val asset = "DnK5Xfi2wXUJx9BjK9X6ZpFdTLdq2GtWH9pWrcxcmrhB"
  val lease = "6Tn7ir9MycHW6Gq2F2dGok2stokSwXJadPh4hW8eZ8Sp"

  var storage: Map[String, DataEntry[_]] = Map.empty[String, DataEntry[_]]

  var balances: Map[String, Map[String, Long]] = Map(
    "null" -> Map(
      "4WVhw3QdiinpE5QXDG7QfqLiLanM7ewBw4ChX4qyGjs2" -> 10000000000L,
      "3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU" -> 10000000000L,
      "3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3" -> 10000000000L,
      "3Nnx8cX3UiyfQeC3YQKVRqVr2ewSxrvaDyB" -> 10000000000L,
      "3NzC4Ex91VBQKfJHPiGhuPEomLg48NMi2ZF" -> 10000000000L,
    ),
    "DnK5Xfi2wXUJx9BjK9X6ZpFdTLdq2GtWH9pWrcxcmrhB" -> Map(
      "4WVhw3QdiinpE5QXDG7QfqLiLanM7ewBw4ChX4qyGjs2" -> 10000000000L,
      "3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU" -> 10000000000L,
      "3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3" -> 10000000000L,
      "3Nnx8cX3UiyfQeC3YQKVRqVr2ewSxrvaDyB" -> 10000000000L,
      "3NzC4Ex91VBQKfJHPiGhuPEomLg48NMi2ZF" -> 10000000000L,
    )
  )
  
  override def getBytecode(contractId: Array[Byte]): Array[Byte] = {
    getClass.getResourceAsStream("/mock.wasm").readAllBytes()
  }

  override def getStorage(contractId: Array[Byte], key: Array[Byte]): Array[Byte] = {
    val k = if (key.isEmpty) throw new Exception else new String(key)
    this.storage.get(k) match {
      case Some(value) => toBytes(value)
      case None => Array.empty[Byte]
    }
  }

  override def setStorage(contractId: Array[Byte], value: Array[Byte]) = {
    if (Base58.encode(contractId) != this.contract) throw new Exception
    val dataEntry = parse(value, 0)._1
    this.storage(dataEntry.key) = dataEntry
  }

  override def getBalance(assetId: Array[Byte], address: Array[Byte]): Long = {
    val as = if (assetId.isEmpty) "null" else Base58.encode(assetId)
    val ad = if (address.isEmpty) throw new Exception else Base58.encode(address)
    this.balances.get(as) match {
      case Some(balances) => balances.get(ad) match {
        case Some(balance) => balance
        case None => 0
      }
      case None => throw new Exception
    }
  }

  override def transfer(contractId: Array[Byte], assetId: Array[Byte], recipient: Array[Byte], amount: Long) = {
    if (Base58.encode(contractId) != this.contract) throw new Exception

    val a = if (assetId.isEmpty) "null" else Base58.encode(assetId)
    val r = if (recipient.isEmpty) throw new Exception else Base58.encode(recipient)

    val balance = this.balances.get(a) match {
      case Some(balances) => balances.get(Base58.encode(contractId)) match {
        case Some(balance) => balance
        case None => 0
      }
      case None => throw new Exception
    }
    if (balance < amount) throw new Exception

    this.balances(a)(Base58.encode(contractId)) -= amount
    this.balances(a)(r) += amount
  }

  override def issue(contractId: Array[Byte], name: Array[Byte], description: Array[Byte], quantity: Long, decimals: Int, isReissuable: Boolean): Array[Byte] = {
    if (Base58.encode(contractId) != this.contract) throw new Exception
    Base58.decode(this.asset).get
  }

  override def burn(contractId: Array[Byte], assetId: Array[Byte], amount: Long) = {
    if (Base58.encode(contractId) != this.contract) throw new Exception
    if (Base58.encode(assetId) != this.asset) throw new Exception

    val balance = this.balances.get(Base58.encode(assetId)) match {
      case Some(balances) => balances.get(Base58.encode(contractId)) match {
        case Some(balance) => balance
        case None => 0
      }
      case None => throw new Exception
    }
    if (balance < amount) throw new Exception

    this.balances(Base58.encode(assetId))(Base58.encode(contractId)) -= amount
  }

  override def reissue(contractId: Array[Byte], assetId: Array[Byte], amount: Long, isReissuable: Boolean) = {
    if (Base58.encode(contractId) != this.contract) throw new Exception
    if (Base58.encode(assetId) != this.asset) throw new Exception

    this.balances(Base58.encode(assetId))(Base58.encode(contractId)) += amount
  }

  override def lease(contractId: Array[Byte], recipient: Array[Byte], amount: Long): Array[Byte] = {
    if (Base58.encode(contractId) != this.contract) throw new Exception
    if (Base58.encode(recipient) == this.recipient) Base58.decode(this.lease).get else throw new Exception
  }

  override def cancelLease(contractId: Array[Byte], leaseId: Array[Byte]) = {
    if (Base58.encode(contractId) != this.contract) throw new Exception
    if (Base58.encode(leaseId) != this.lease) throw new Exception
  }

  override def getBlockTimestamp: Long = 1690202857485L

  override def getBlockHeight: Long = 3745592L

  override def getTxSender: Array[Byte] =
    Base58.decode(this.txSender).get

  override def getTxPayments(contractId: Array[Byte]): Int =
    if (Base58.encode(contractId) != this.contract) throw new Exception else 2

  override def getTxPaymentAssetId(contractId: Array[Byte], number: Int): Array[Byte] = {
    if (Base58.encode(contractId) != this.contract) throw new Exception
    if (number == 1) Base58.decode(this.asset).get else Array.empty[Byte]
  } 

  override def getTxPaymentAmount(contractId: Array[Byte], number: Int): Long = {
    if (Base58.encode(contractId) != this.contract) throw new Exception
    if (number == 1) 2400000000L else 4200000000L
  }
}
