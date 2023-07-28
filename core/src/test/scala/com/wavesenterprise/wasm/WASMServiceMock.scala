package com.wavesenterprise.wasm

class WASMServiceMock extends WASMService {
  override def getBytecode(contractId: Array[Byte]): Array[Byte] = {
    // (module
    //     (func (export "_constructor"))
    //     (func (export "sum") (param $p0 i64) (result i32)
    //         (i32.ne
    //             (i32.add
    //                 (i32.const 2)
    //                 (i32.wrap_i64
    //                     (local.get $p0)))
    //             (i32.const 4))
    //         )
    //     )
    Array[Byte](
      0, 97, 115, 109, 1, 0, 0, 0, 1, 9, 2, 96, 0, 0, 96, 1, 126, 1, 127, 3, 3, 2,
      0, 1, 7, 22, 2, 12, 95, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
      0, 0, 3, 115, 117, 109, 0, 1, 10, 16, 2, 2, 0, 11, 11, 0, 65, 2, 32, 0, 167.toByte,
      106, 65, 4, 71, 11, 0, 14, 4, 110, 97, 109, 101, 2, 7, 1, 1, 1, 0, 2, 112, 48
    )
  }

  override def getStorage(contractId: Array[Byte], key: Array[Byte]): Array[Byte] = Array.empty[Byte]

  override def setStorage(key: Array[Byte], dataType: String, value: Array[Byte]): Boolean = false

  override def getBalance(assetId: Array[Byte], address: Array[Byte]): Long = 0

  override def transfer(assetId: Array[Byte], recipient: Array[Byte], amount: Long): Boolean = false

  override def issue(name: Array[Byte], description: Array[Byte], quantity: Long, decimals: Int, isReissuable: Boolean): Array[Byte] = Array.empty[Byte]

  override def burn(assetId: Array[Byte], amount: Long): Boolean = false

  override def reissue(assetId: Array[Byte], amount: Long, isReissuable: Boolean): Boolean = false

  override def lease(recipient: Array[Byte], amount: Long): Array[Byte] = Array.empty[Byte]

  override def cancelLease(leaseId: Array[Byte]): Boolean = false

  override def getBlockTimestamp: Long = 0

  override def getBlockHeight: Long = 0

  override def getTxSender: Array[Byte] = Array.empty[Byte]

  override def getTxPayments: Int = 0

  override def getTxPaymentAssetId(number: Int): Array[Byte] = Array.empty[Byte]

  override def getTxPaymentAmount(number: Int): Long = 0
}
