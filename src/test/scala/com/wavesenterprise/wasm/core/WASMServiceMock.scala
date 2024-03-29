package com.wavesenterprise.wasm.core

import com.google.common.primitives.Longs

import com.wavesenterprise.crypto.internals.WavesAlgorithms
import com.wavesenterprise.state.DataEntry
import com.wavesenterprise.transaction.docker.ContractTransactionEntryOps.{parse, toBytes}
import com.wavesenterprise.utils.Base58

import java.nio.charset.StandardCharsets.UTF_8

import scala.collection.mutable.{Map, Seq}

class WASMServiceMock extends WASMService {
  val contract        = "4WVhw3QdiinpE5QXDG7QfqLiLanM7ewBw4ChX4qyGjs2"
  val contractMock    = "2sqPS2VAKmK77FoNakw1VtDTCbDSa7nqh5wTXvJeYGo2"
  val contractStorage = "757aQzJiQZRfVRuJNnP3L1d369H2oTjUEazwtYxGngCd"

  val txSender  = "3NqEjAkFVzem9CGa3bEPhakQc1Sm2G8gAFU"
  val recipient = "3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3"
  val alias     = "miner"
  val asset     = "DnK5Xfi2wXUJx9BjK9X6ZpFdTLdq2GtWH9pWrcxcmrhB"
  val lease     = "6Tn7ir9MycHW6Gq2F2dGok2stokSwXJadPh4hW8eZ8Sp"

  val paymentIdContract = Base58.encode(Base58.decode(this.contract).get ++ Array.fill[Byte](8)(0))
  val paymentIdMock     = Base58.encode(Base58.decode(this.contractMock).get ++ Array[Byte](0, 0, 0, 0, 0, 0, 0, 1))
  val paymentIdStorage  = Base58.encode(Base58.decode(this.contractStorage).get ++ Array[Byte](0, 0, 0, 0, 0, 0, 0, 1))

  var storage: Map[String, Map[String, DataEntry[_]]] = Map(
    this.contract        -> Map.empty[String, DataEntry[_]],
    this.contractMock    -> Map.empty[String, DataEntry[_]],
    this.contractStorage -> Map.empty[String, DataEntry[_]],
  )

  var balances: Map[String, Map[String, Long]] = Map(
    "null" -> Map(
      this.contract        -> 10000000000L,
      this.contractMock    -> 10000000000L,
      this.contractStorage -> 10000000000L,
      this.txSender        -> 10000000000L,
      this.recipient       -> 10000000000L,
    ),
    this.asset -> Map(
      this.contract        -> 10000000000L,
      this.contractMock    -> 10000000000L,
      this.contractStorage -> 10000000000L,
      this.txSender        -> 10000000000L,
      this.recipient       -> 10000000000L,
    )
  )

  var payments: Map[String, Seq[(String, Long)]] = Map(
    paymentIdContract -> Seq(
      ("null", 4200000000L),
      (this.asset, 2400000000L),
    ),
    paymentIdMock    -> Seq.empty[(String, Long)],
    paymentIdStorage -> Seq.empty[(String, Long)],
  )

  private def parseAssetHolder(bytes: Array[Byte]): (Int, Int, String) = {
    val `type`  = bytes(0)
    val version = bytes(1)
    val chainId = bytes(2)

    val assetHolder = `type` match {
      case 0 => version match {
          case 1       => if (checkChainId(chainId)) Base58.encode(bytes.drop(1)) else throw new Exception
          case 2       => if (checkChainId(chainId)) new String(bytes.drop(3), UTF_8) else throw new Exception
          case _: Byte => throw new Exception
        }
      case 1       => Base58.encode(bytes.drop(1))
      case _: Byte => throw new Exception
    }

    (`type`, version, assetHolder)
  }

  private def checkChainId(byte: Byte): Boolean = {
    byte == getChainId()
  }

  override def getChainId(): Byte = 'V'.toByte

  override def getBytecode(contractId: Array[Byte]): Array[Byte] = Base58.encode(contractId) match {
    case this.contractMock    => getClass.getResourceAsStream("/mock.wasm").readAllBytes()
    case this.contractStorage => getClass.getResourceAsStream("/storage.wasm").readAllBytes()
    case _                    => throw new Exception
  }

  override def addPayments(contractId: Array[Byte], paymentId: Array[Byte], payments: Array[Byte]) = {
    if (payments.isEmpty) throw new Exception

    val assetLength = 32
    var start       = 2

    val callableContractId: Array[Byte] = Array[Byte](1) ++ paymentId.slice(0, 32)

    var count: Int = ((payments(0) & 0xff) << 8) | (payments(1) & 0xff)
    while (count > 0) {
      var assetIdBytes = Array.empty[Byte]

      if (payments(start) == 1) {
        assetIdBytes = payments.slice(start + 1, start + 1 + assetLength)
        start += 1 + assetLength
      } else if (payments(start) == 0) {
        start += 1
      } else {
        throw new Exception
      }

      val amount = Longs.fromByteArray(payments.slice(start, start + Longs.BYTES))
      start += Longs.BYTES

      transfer(contractId, assetIdBytes, callableContractId, amount)

      val assetId                 = if (assetIdBytes.isEmpty) "null" else Base58.encode(assetIdBytes)
      val payment: (String, Long) = (assetId, amount)
      this.payments(Base58.encode(paymentId)) = this.payments(Base58.encode(paymentId)) :+ payment

      count -= 1
    }
  }

  override def getBalance(assetId: Array[Byte], assetHolder: Array[Byte]): Long = {
    val a              = if (assetId.isEmpty) "null" else Base58.encode(assetId)
    val (_, _, holder) = parseAssetHolder(assetHolder)

    this.balances.get(a) match {
      case Some(balances) => balances.get(holder) match {
          case Some(balance) => balance
          case None          => 0
        }
      case None => throw new Exception
    }
  }

  override def transfer(contractId: Array[Byte], assetId: Array[Byte], recipient: Array[Byte], amount: Long) = {
    val a              = if (assetId.isEmpty) "null" else Base58.encode(assetId)
    val (_, _, holder) = parseAssetHolder(recipient)

    val balance = this.balances.get(a) match {
      case Some(balances) => balances.get(Base58.encode(contractId)) match {
          case Some(balance) => balance
          case None          => 0
        }
      case None => throw new Exception
    }
    if (balance < amount) throw new Exception

    this.balances(a)(Base58.encode(contractId)) -= amount
    this.balances(a)(holder) += amount
  }

  override def issue(contractId: Array[Byte],
                     name: Array[Byte],
                     description: Array[Byte],
                     quantity: Long,
                     decimals: Long,
                     isReissuable: Boolean): Array[Byte] =
    Base58.decode(this.asset).get

  override def burn(contractId: Array[Byte], assetId: Array[Byte], amount: Long) = {
    val balance = this.balances.get(Base58.encode(assetId)) match {
      case Some(balances) => balances.get(Base58.encode(contractId)) match {
          case Some(balance) => balance
          case None          => 0
        }
      case None => throw new Exception
    }
    if (balance < amount) throw new Exception

    this.balances(Base58.encode(assetId))(Base58.encode(contractId)) -= amount
  }

  override def reissue(contractId: Array[Byte], assetId: Array[Byte], amount: Long, isReissuable: Boolean) =
    this.balances(Base58.encode(assetId))(Base58.encode(contractId)) += amount

  override def getBlockTimestamp: Long = 1690202857485L

  override def getBlockHeight: Long = 3745592L

  override def fastHash(bytes: Array[Byte]): Array[Byte] = WavesAlgorithms.fastHash(bytes)

  override def secureHash(bytes: Array[Byte]): Array[Byte] = WavesAlgorithms.secureHash(bytes)

  override def sigVerify(message: Array[Byte], signature: Array[Byte], publicKey: Array[Byte]): Boolean =
    WavesAlgorithms.verify(signature, message, publicKey)

  override def lease(contractId: Array[Byte], recipient: Array[Byte], amount: Long): Array[Byte] = {
    val assetHolder = parseAssetHolder(recipient)

    val `type`  = assetHolder._1
    val version = assetHolder._2
    val holder  = assetHolder._3

    if (`type` == 1) throw new Exception

    version match {
      case 1 => if (holder != this.recipient) throw new Exception
      case 2 => if (holder != this.alias) throw new Exception
      case _ => throw new Exception
    }

    Base58.decode(this.lease).get
  }

  override def cancelLease(contractId: Array[Byte], leaseId: Array[Byte]) =
    if (Base58.encode(leaseId) != this.lease) throw new Exception

  override def containsKey(contractId: Array[Byte], key: Array[Byte]): Boolean = {
    val k = if (key.isEmpty) throw new Exception else new String(key)
    this.storage.get(Base58.encode(contractId)) match {
      case Some(kv) => kv.contains(k)
      case None     => throw new Exception
    }
  }

  override def getStorage(contractId: Array[Byte], key: Array[Byte]): Array[Byte] = {
    val k = if (key.isEmpty) throw new Exception else new String(key)
    this.storage.get(Base58.encode(contractId)) match {
      case Some(kv) => kv.get(k) match {
          case Some(value) => toBytes(value)
          case None        => Array.empty[Byte]
        }
      case None => throw new Exception
    }
  }

  override def setStorage(contractId: Array[Byte], value: Array[Byte]) = {
    val dataEntry = parse(value, 0)._1
    this.storage(Base58.encode(contractId))(dataEntry.key) = dataEntry
  }

  override def getTxPayments(paymentId: Array[Byte]): Long =
    this.payments(Base58.encode(paymentId)).size

  override def getTxPaymentAssetId(paymentId: Array[Byte], number: Long): Array[Byte] =
    this.payments.get(Base58.encode(paymentId)) match {
      case Some(seq) => {
        val assetId = seq.apply(number.toInt)._1
        if (assetId == "null") Array.empty[Byte] else Base58.decode(assetId).get
      }
      case None => throw new Exception
    }

  override def getTxPaymentAmount(paymentId: Array[Byte], number: Long): Long =
    this.payments.get(Base58.encode(paymentId)) match {
      case Some(seq) => seq.apply(number.toInt)._2
      case None      => throw new Exception
    }

  override def tx(field: Array[Byte]): Array[Byte] = {
    val string = new String(field, UTF_8)
    if (string == "sender") {
      Base58.decode(this.txSender).get
    } else {
      throw new Exception
    }
  }
}
