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
    * @param contractId ID of a contract called this function. Base58 bytes
    * @param paymentId Unique payment identifier. Represents the concatenation of contractId bytes and unique 8 bytes
    * @param payments Serialized list assetId and amount
    */
  def addPayments(contractId: Array[Byte], paymentId: Array[Byte], payments: Array[Byte])

  /**
    * @param assetId ID of a token (optional field, array can be empty). Base58 bytes
    * @param assetHolder AssetHolder of the token holder (possible contractId called this function)
    * @return Amount of tokens
    */
  def getBalance(assetId: Array[Byte], assetHolder: Array[Byte]): Long

  /**
    * @param contractId ID of a contract called this function. Base58 bytes
    * @param assetId ID of a token to be transferred (optional field, array can be empty). Base58 bytes
    * @param recipient AssetHolder of recipient of tokens
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
  def issue(contractId: Array[Byte], name: Array[Byte], description: Array[Byte], quantity: Long, decimals: Long, isReissuable: Boolean): Array[Byte]

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
    * @param field UTF-8 string with block field name
    * @return Requested field data
    */
  def block(field: Array[Byte]): Array[Byte]

  /**
    * @param bytes Raw data
    * @return Cryptographic hash
    */
  def fastHash(bytes: Array[Byte]): Array[Byte]

  /**
    * @param bytes Raw data
    * @return Cryptographic hash
    */
  def secureHash(bytes: Array[Byte]): Array[Byte]

  /**
    * @param message Raw message
    * @param signature Cryptographic signature
    * @param publicKey Cryptographic Public Key
    * @return True if the signature was a valid signature
    */
  def sigVerify(message: Array[Byte], signature: Array[Byte], publicKey: Array[Byte]): Boolean

  /**
    * @param contractId ID of a contract called this function. Base58 bytes
    * @param recipient AssetHolder of recipient of tokens
    * @param amount Number of tokens for leasing
    * @return leaseId of a leasing transaction. Base58 bytes
    */
  def lease(contractId: Array[Byte], recipient: Array[Byte], amount: Long): Array[Byte]

  /**
    * @param contractId ID of a contract called this function. Base58 bytes
    * @param leaseId ID of a leasing transaction. Base58 bytes
    */
  def cancelLease(contractId: Array[Byte], leaseId: Array[Byte])

  /**
    * @param contractId ID of a contract (possible contractId called this function). Base58 bytes
    * @param key Record key. UTF-8 bytes
    * @return Boolean value whether the key exists
    */
  def containsKey(contractId: Array[Byte], key: Array[Byte]): Boolean

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
    * @param paymentId Unique payment identifier. Represents the concatenation of contractId bytes and unique 8 bytes
    * @return Number of attached payments
    */
  def getTxPayments(paymentId: Array[Byte]): Long

  /**
    * @param paymentId Unique payment identifier. Represents the concatenation of contractId bytes and unique 8 bytes
    * @param number Attached payment number
    * @return assetId of a token (optional field, array can be empty). Base58 bytes
    */
  def getTxPaymentAssetId(paymentId: Array[Byte], number: Long): Array[Byte]

  /**
    * @param paymentId Unique payment identifier. Represents the concatenation of contractId bytes and unique 8 bytes
    * @param number Attached payment number
    * @return Amount of tokens
    */
  def getTxPaymentAmount(paymentId: Array[Byte], number: Long): Long

  /**
    * @param field UTF-8 string with transaction field name
    * @return Requested field data
    */
  def tx(field: Array[Byte]): Array[Byte]
}
