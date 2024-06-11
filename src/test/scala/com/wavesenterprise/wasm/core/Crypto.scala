package com.wavesenterprise.wasm.core

import org.bouncycastle.crypto.Digest
import org.bouncycastle.crypto.digests.{Blake2bDigest, KeccakDigest, SHA256Digest}

abstract class BCDigest(digest: Digest, digestSize: Int) {
  def hash(message: Array[Byte]): Array[Byte] = {
    digest.update(message, 0, message.length)
    val result = new Array[Byte](digestSize)
    digest.doFinal(result, 0)
    result
  }
}

object Blake2b256 extends BCDigest(new Blake2bDigest(256), 32)
object Keccak256  extends BCDigest(new KeccakDigest(256), 32)
object SHA256     extends BCDigest(new SHA256Digest(), 32)
