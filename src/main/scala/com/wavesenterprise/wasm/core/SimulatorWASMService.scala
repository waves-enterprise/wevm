package com.wavesenterprise.wasm.core

import com.google.common.primitives.Longs

import com.wavesenterprise.crypto.internals.WavesAlgorithms
import com.wavesenterprise.state.DataEntry
import com.wavesenterprise.transaction.docker.ContractTransactionEntryOps.{parse, toBytes}

import java.nio.ByteBuffer
import java.nio.charset.StandardCharsets.UTF_8
import java.time.Instant

import scala.collection.mutable.{Map, Seq}

class SimulatorWASMService extends WASMService {
  private var _chainId: Byte         = 'V'.toByte
  private var _timestamp: Long       = Instant.now().toEpochMilli()
  private val _height: Long          = 1L
  private var _txSender: Array[Byte] = Array.empty[Byte]

  private val _bytecodes: Map[ByteBuffer, Array[Byte]]             = Map.empty[ByteBuffer, Array[Byte]]
  private val _balances: Map[ByteBuffer, Map[ByteBuffer, Long]]    = Map.empty[ByteBuffer, Map[ByteBuffer, Long]]
  private val _leases: Map[ByteBuffer, (ByteBuffer, Long)]         = Map.empty[ByteBuffer, (ByteBuffer, Long)]
  private val _storage: Map[ByteBuffer, Map[String, DataEntry[_]]] = Map.empty[ByteBuffer, Map[String, DataEntry[_]]]
  private val _payments: Map[ByteBuffer, Seq[(ByteBuffer, Long)]]  = Map.empty[ByteBuffer, Seq[(ByteBuffer, Long)]]

  private[core] def setChainId(value: Byte) = this._chainId = value

  private[core] def timestamp: Long           = this._timestamp
  private[core] def setTimestamp(value: Long) = this._timestamp = value

  private[core] def height: Long = this._height

  private[core] def setTxSender(value: Array[Byte]) = this._txSender = value

  private[core] def setBytecode(contractId: ByteBuffer, bytecode: Array[Byte]) =
    this._bytecodes(contractId) = bytecode

  private def addPayment(paymentId: ByteBuffer, payment: (ByteBuffer, Long)) =
    this._payments(paymentId) = this._payments.getOrElse(paymentId, Seq.empty[(ByteBuffer, Long)]) :+ payment

  private[core] def addPayment(contractId: Array[Byte], assetId: Array[Byte], amount: Long): Unit = {
    val paymentId = ByteBuffer.wrap(contractId ++ Array.fill[Byte](8)(0))
    this.addPayment(paymentId, (ByteBuffer.wrap(assetId), amount))
  }

  private def parseAssetHolder(bytes: Array[Byte]): (Int, Int, Array[Byte]) = {
    val `type`  = bytes(0)
    val version = bytes(1)
    val chainId = bytes(2)

    val assetHolder = `type` match {
      case 0 => version match {
          case 1       => if (checkChainId(chainId)) bytes.drop(1) else throw new Exception
          case 2       => if (checkChainId(chainId)) bytes.drop(3) else throw new Exception
          case _: Byte => throw new Exception
        }
      case 1       => bytes.drop(1)
      case _: Byte => throw new Exception
    }

    (`type`, version, assetHolder)
  }

  private def checkChainId(byte: Byte): Boolean = {
    byte == getChainId()
  }

  private[core] def getBalance(assetId: ByteBuffer, assetHolder: ByteBuffer): Long =
    this._balances.get(assetId) match {
      case Some(balances) => balances.getOrElse(assetHolder, 0L)
      case None           => throw new Exception
    }

  private[core] def updateBalance(assetId: ByteBuffer, assetHolder: ByteBuffer, amount: Long) = {
    val balances = this._balances.getOrElse(assetId, Map.empty[ByteBuffer, Long])
    balances(assetHolder) = amount
    this._balances(assetId) = balances
  }

  private[core] def transfer(sender: ByteBuffer, assetId: ByteBuffer, recipient: ByteBuffer, amount: Long) = {
    val balanceSender = this.getBalance(assetId, sender)
    if (balanceSender < amount) throw new Exception

    val balanceRecipient = this.getBalance(assetId, recipient)

    this.updateBalance(assetId, sender, balanceSender - amount)
    this.updateBalance(assetId, recipient, balanceRecipient + amount)
  }

  private def longToBytes(value: Long): Array[Byte] =
    ByteBuffer.allocate(8).putLong(value).array()

  private def getKeyValueStorage(contractId: ByteBuffer): Map[String, DataEntry[_]] =
    this._storage.getOrElse(contractId, Map.empty[String, DataEntry[_]])

  override def getChainId(): Byte = this._chainId

  override def getBytecode(contractId: Array[Byte]): Array[Byte] =
    this._bytecodes(ByteBuffer.wrap(contractId))

  override def addPayments(contractId: Array[Byte], paymentId: Array[Byte], payments: Array[Byte]) = {
    if (payments.isEmpty) throw new Exception

    val assetLength = 32
    var start       = 2

    val callableContractId: Array[Byte] = Array[Byte](1) ++ paymentId.slice(0, 32)

    var count: Int = ((payments(0) & 0xff) << 8) | (payments(1) & 0xff)
    while (count > 0) {
      var assetId = Array.empty[Byte]

      if (payments(start) == 1) {
        assetId = payments.slice(start + 1, start + 1 + assetLength)
        start += 1 + assetLength
      } else if (payments(start) == 0) {
        start += 1
      } else {
        throw new Exception
      }

      val amount = Longs.fromByteArray(payments.slice(start, start + Longs.BYTES))
      start += Longs.BYTES

      this.transfer(contractId, assetId, callableContractId, amount)

      val payment: (ByteBuffer, Long) = (ByteBuffer.wrap(assetId), amount)
      this.addPayment(ByteBuffer.wrap(paymentId), payment)

      count -= 1
    }
  }

  override def getBalance(assetId: Array[Byte], assetHolder: Array[Byte]): Long = {
    val (_, _, holder) = parseAssetHolder(assetHolder)
    this.getBalance(ByteBuffer.wrap(assetId), ByteBuffer.wrap(holder))
  }

  override def transfer(contractId: Array[Byte], assetId: Array[Byte], recipient: Array[Byte], amount: Long) = {
    val (_, _, holder) = parseAssetHolder(recipient)

    val wContractId = ByteBuffer.wrap(contractId)
    val wAssetId    = ByteBuffer.wrap(assetId)
    val wHolder     = ByteBuffer.wrap(holder)

    this.transfer(wContractId, wAssetId, wHolder, amount)
  }

  override def issue(contractId: Array[Byte],
                     name: Array[Byte],
                     description: Array[Byte],
                     quantity: Long,
                     decimals: Long,
                     isReissuable: Boolean): Array[Byte] = {
    val assetId = WavesAlgorithms.secureHash(name ++ description)
    this.updateBalance(ByteBuffer.wrap(assetId), ByteBuffer.wrap(contractId), quantity)
    assetId
  }

  override def burn(contractId: Array[Byte], assetId: Array[Byte], amount: Long) = {
    val wContractId = ByteBuffer.wrap(contractId)
    val wAssetId    = ByteBuffer.wrap(assetId)

    val balance = this.getBalance(wAssetId, wContractId)
    if (balance < amount) throw new Exception

    this.updateBalance(wAssetId, wContractId, balance - amount)
  }

  override def reissue(contractId: Array[Byte], assetId: Array[Byte], amount: Long, isReissuable: Boolean) = {
    val wContractId = ByteBuffer.wrap(contractId)
    val wAssetId    = ByteBuffer.wrap(assetId)

    val balance = this.getBalance(wAssetId, wContractId)
    this.updateBalance(wAssetId, wContractId, balance + amount)
  }

  override def block(field: Array[Byte]): Array[Byte] =
    new String(field, UTF_8) match {
      case "timestamp" => longToBytes(this._timestamp)
      case "height"    => longToBytes(this._height)
      case _           => throw new Exception
    }

  override def fastHash(bytes: Array[Byte]): Array[Byte] = WavesAlgorithms.fastHash(bytes)

  override def secureHash(bytes: Array[Byte]): Array[Byte] = WavesAlgorithms.secureHash(bytes)

  override def sigVerify(message: Array[Byte], signature: Array[Byte], publicKey: Array[Byte]): Boolean =
    WavesAlgorithms.verify(signature, message, publicKey)

  override def lease(contractId: Array[Byte], recipient: Array[Byte], amount: Long): Array[Byte] = {
    val assetHolder = parseAssetHolder(recipient)

    val `type` = assetHolder._1
    val holder = assetHolder._3

    if (`type` == 1) throw new Exception

    val leaseId = WavesAlgorithms.secureHash(contractId ++ recipient)

    this._leases(ByteBuffer.wrap(leaseId)) = (ByteBuffer.wrap(holder), amount)

    leaseId
  }

  override def cancelLease(contractId: Array[Byte], leaseId: Array[Byte]) =
    this._leases.remove(ByteBuffer.wrap(leaseId)) match {
      case None => throw new Exception
      case _    => ()
    }

  override def containsKey(contractId: Array[Byte], key: Array[Byte]): Boolean = {
    val k = if (key.isEmpty) throw new Exception else new String(key)
    this.getKeyValueStorage(ByteBuffer.wrap(contractId)).contains(k)
  }

  override def getStorage(contractId: Array[Byte], key: Array[Byte]): Array[Byte] = {
    val k = if (key.isEmpty) throw new Exception else new String(key)

    this.getKeyValueStorage(ByteBuffer.wrap(contractId)).get(k) match {
      case Some(value) => toBytes(value)
      case None        => Array.empty[Byte]
    }
  }

  override def setStorage(contractId: Array[Byte], value: Array[Byte]) = {
    val wContractId             = ByteBuffer.wrap(contractId)
    val dataEntry: DataEntry[_] = parse(value, 0)._1

    val kv = this.getKeyValueStorage(wContractId)
    kv(dataEntry.key) = dataEntry
    this._storage(wContractId) = kv
  }

  override def getTxPayments(paymentId: Array[Byte]): Long =
    this._payments.getOrElse(ByteBuffer.wrap(paymentId), Seq.empty[(ByteBuffer, Long)]).size

  override def getTxPaymentAssetId(paymentId: Array[Byte], number: Long): Array[Byte] =
    this._payments.get(ByteBuffer.wrap(paymentId)) match {
      case Some(seq) => seq.apply(number.toInt)._1.array
      case None      => throw new Exception
    }

  override def getTxPaymentAmount(paymentId: Array[Byte], number: Long): Long =
    this._payments.get(ByteBuffer.wrap(paymentId)) match {
      case Some(seq) => seq.apply(number.toInt)._2
      case None      => throw new Exception
    }

  override def tx(field: Array[Byte]): Array[Byte] =
    new String(field, UTF_8) match {
      case "sender" => this._txSender
      case _        => throw new Exception
    }
}
