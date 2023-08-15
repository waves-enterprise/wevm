package com.wavesenterprise.wasm

trait WASMService {

  /**
    * @param contractId ID of a contract. Base58 bytes
    * @return Bytecode contract
    */
  def getBytecode(contractId: Array[Byte]): Array[Byte]

  /**
    * @param contractId ID of a contract (optional field, array can be empty). Base58 bytes
    * @param key Record key. UTF-8 bytes
    * @return Record value
    */
  def getStorage(contractId: Array[Byte], key: Array[Byte]): Array[Byte]

  /**
    * @param data Serialized DataEntry record value
    */
  def setStorage(data: Array[Byte])

  /**
    * @param assetId ID of a token (optional field, array can be empty). Base58 bytes
    * @param address Address of the token holder. Base58 bytes
    * @return Amount of tokens
    */
  def getBalance(assetId: Array[Byte], address: Array[Byte]): Long

  /**
    * @param assetId ID of a token to be transferred (optional field, array can be empty). Base58 bytes
    * @param recipient Address of recipient of tokens. Base58 bytes
    * @param amount Amount of tokens
    */
  def transfer(assetId: Array[Byte], recipient: Array[Byte], amount: Long)

  /**
    * @param name An arbitrary name of asset. UTF-8 bytes
    * @param description An arbitrary description of a asset. UTF-8 bytes
    * @param quantity Number of tokens to be issued
    * @param decimals Digit capacity of a token in use
    * @param isReissuable Re-issuability of a token
    * @return assetId. Base58 bytes
    */
  def issue(name: Array[Byte], description: Array[Byte], quantity: Long, decimals: Int, isReissuable: Boolean): Array[Byte]

  /**
    * @param assetId ID of a token to be burned. Base58 bytes
    * @param amount Amount of tokens
    */
  def burn(assetId: Array[Byte], amount: Long)

  /**
    * @param assetId ID of a token to be reissued. Base58 bytes
    * @param amount Amount of tokens
    * @param isReissuable Re-issuability of a token
    */
  def reissue(assetId: Array[Byte], amount: Long, isReissuable: Boolean)

  /**
    * @param recipient Address of recipient of tokens. Base58 bytes
    * @param amount Number of tokens for leasing
    * @return leaseId of a leasing transaction. Base58 bytes
    */
  def lease(recipient: Array[Byte], amount: Long): Array[Byte]

  /**
    * @param leaseId ID of a leasing transaction. Base58 bytes
    */
  def cancelLease(leaseId: Array[Byte])

  def getBlockTimestamp: Long

  def getBlockHeight: Long

  /**
    * @return Address calling contract
    */
  def getTxSender: Array[Byte]

  /**
    * @return Number of attached payments
    */
  def getTxPayments: Int

  /**
    * @param number Attached payment number
    * @return assetId of a token (optional field, array can be empty). Base58 bytes
    */
  def getTxPaymentAssetId(number: Int): Array[Byte]

  /**
    * @param number Attached payment number
    * @return Amount of tokens
    */
  def getTxPaymentAmount(number: Int): Long
}
