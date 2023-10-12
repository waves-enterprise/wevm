package com.wavesenterprise.wasm.core

trait WASMService {

  /**
    * @return Blockchain Id
    */
  def getChainId(): Byte

  /**
    * @param contractId ID of a contract. Base58 bytes
    * @return Bytecode contract
    */
  def getBytecode(contractId: Array[Byte]): Array[Byte]

  /**
    * @param contractId ID of a contract. Base58 bytes
    * @param payments Serialized list assetId and amount
    */
  def addPayments(contractId: Array[Byte], payments: Array[Byte])

  /**
    * @param contractId ID of a contract (possible contractId called this function). Base58 bytes
    * @param key Record key. UTF-8 bytes
    * @return Record value
    */
  def getStorage(contractId: Array[Byte], key: Array[Byte]): Array[Byte]

  /**
    * @param contractId ID of a contract called this function. Base58 bytes
    * @param value Serialized DataEntry record value
    */
  def setStorage(contractId: Array[Byte], value: Array[Byte])

  /**
    * @param assetId ID of a token (optional field, array can be empty). Base58 bytes
    * @param address Address or ContractId of the token holder (possible contractId called this function). Base58 bytes
    * @return Amount of tokens
    */
  def getBalance(assetId: Array[Byte], address: Array[Byte]): Long

  /**
    * @param contractId ID of a contract called this function. Base58 bytes
    * @param assetId ID of a token to be transferred (optional field, array can be empty). Base58 bytes
    * @param recipient Address of recipient of tokens. Base58 bytes
    * @param amount Amount of tokens
    */
  def transfer(contractId: Array[Byte], assetId: Array[Byte], recipient: Array[Byte], amount: Long)

  /**
    * @param contractId ID of a contract called this function. Base58 bytes
    * @param name An arbitrary name of asset. UTF-8 bytes
    * @param description An arbitrary description of a asset. UTF-8 bytes
    * @param quantity Number of tokens to be issued
    * @param decimals Digit capacity of a token in use
    * @param isReissuable Re-issuability of a token
    * @return assetId. Base58 bytes
    */
  def issue(contractId: Array[Byte], name: Array[Byte], description: Array[Byte], quantity: Long, decimals: Int, isReissuable: Boolean): Array[Byte]

  /**
    * @param contractId ID of a contract called this function. Base58 bytes
    * @param assetId ID of a token to be burned. Base58 bytes
    * @param amount Amount of tokens
    */
  def burn(contractId: Array[Byte], assetId: Array[Byte], amount: Long)

  /**
    * @param contractId ID of a contract called this function. Base58 bytes
    * @param assetId ID of a token to be reissued. Base58 bytes
    * @param amount Amount of tokens
    * @param isReissuable Re-issuability of a token
    */
  def reissue(contractId: Array[Byte], assetId: Array[Byte], amount: Long, isReissuable: Boolean)

  /**
    * @param contractId ID of a contract called this function. Base58 bytes
    * @param recipient Address of recipient of tokens. Base58 bytes
    * @param amount Number of tokens for leasing
    * @return leaseId of a leasing transaction. Base58 bytes
    */
  def lease(contractId: Array[Byte], recipient: Array[Byte], amount: Long): Array[Byte]

  /**
    * @param contractId ID of a contract called this function. Base58 bytes
    * @param leaseId ID of a leasing transaction. Base58 bytes
    */
  def cancelLease(contractId: Array[Byte], leaseId: Array[Byte])

  def getBlockTimestamp: Long

  def getBlockHeight: Long

  /**
    * @return Address calling contract
    */
  def getTxSender: Array[Byte]

  /**
    * @param contractId ID of a contract called this function. Base58 bytes
    * @return Number of attached payments
    */
  def getTxPayments(contractId: Array[Byte]): Int

  /**
    * @param contractId ID of a contract called this function. Base58 bytes
    * @param number Attached payment number
    * @return assetId of a token (optional field, array can be empty). Base58 bytes
    */
  def getTxPaymentAssetId(contractId: Array[Byte], number: Int): Array[Byte]

  /**
    * @param contractId ID of a contract called this function. Base58 bytes
    * @param number Attached payment number
    * @return Amount of tokens
    */
  def getTxPaymentAmount(contractId: Array[Byte], number: Int): Long
}
