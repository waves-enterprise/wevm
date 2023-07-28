package com.wavesenterprise.wasm

/**
  * Fields contractId, assetId, leaseId, address, recipient are serialized strings
  * "3NkZd8Xd4KsuPiNVsuphRNCZE3SqJycqv8d" -> Array[Byte]
  */

trait WASMService {

  /**
    * @param contractId ID of a contract
    * @return Bytecode contract
    */
  def getBytecode(contractId: Array[Byte]): Array[Byte]

  /**
    * @param contractId ID of a contract (optional field, array can be empty)
    * @param key Record key
    * @return Record value
    */
  def getStorage(contractId: Array[Byte], key: Array[Byte]): Array[Byte]

  /**
    * @param contractId ID of a contract
    * @param key Record key
    * @param dataType Record data type. Possible values: `binary` `bool` `integer` `string` and `null` (record deletion by its key)
    * @param value Record value
    * @return Execution result (true/false)
    */
  def setStorage(contractId: Array[Byte], key: Array[Byte], dataType: String, value: Array[Byte]): Boolean

  /**
    * @param assetId ID of a token (optional field, array can be empty)
    * @param address Address of the token holder
    * @return Amount of tokens
    */
  def getBalance(assetId: Array[Byte], address: Array[Byte]): Long

  /**
    * @param assetId ID of a token to be transferred (optional field, array can be empty)
    * @param recipient Address of recipient of tokens
    * @param amount Amount of tokens
    * @return Execution result (true/false)
    */
  def transfer(assetId: Array[Byte], recipient: Array[Byte], amount: Long): Boolean

  /**
    * @param name An arbitrary name of asset
    * @param description An arbitrary description of a asset
    * @param quantity Number of tokens to be issued
    * @param decimals Digit capacity of a token in use
    * @param isReissuable Re-issuability of a token
    * @return assetId
    */
  def issue(name: Array[Byte], description: Array[Byte], quantity: Long, decimals: Int, isReissuable: Boolean): Array[Byte]

  /**
    * @param assetId ID of a token to be burned
    * @param amount Amount of tokens
    * @return Execution result (true/false)
    */
  def burn(assetId: Array[Byte], amount: Long): Boolean

  /**
    * @param assetId ID of a token to be reissued
    * @param amount Amount of tokens
    * @param isReissuable Re-issuability of a token
    * @return Execution result (true/false)
    */
  def reissue(assetId: Array[Byte], amount: Long, isReissuable: Boolean): Boolean

  /**
    * @param recipient Address of recipient of tokens
    * @param amount Number of tokens for leasing
    * @return leaseId of a leasing transaction
    */
  def lease(recipient: Array[Byte], amount: Long): Array[Byte]

  /**
    * @param leaseId ID of a leasing transaction
    * @return Execution result (true/false)
    */
  def cancelLease(leaseId: Array[Byte]): Boolean

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
    * @return assetId of a token (optional field, array can be empty)
    */
  def getTxPaymentAssetId(number: Int): Array[Byte]

  /**
    * @param number Attached payment number
    * @return Amount of tokens
    */
  def getTxPaymentAmount(number: Int): Long
}
