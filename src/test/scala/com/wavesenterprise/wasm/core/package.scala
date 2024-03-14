package com.wavesenterprise.wasm

import com.google.common.io.ByteArrayDataOutput
import com.wavesenterprise.serialization.BinarySerializer
import com.wavesenterprise.state.DataEntry
import com.wavesenterprise.transaction.docker.ContractTransactionEntryOps.toBytes

package object core {
  // Fuel limit for tests
  def fuelLimit: Long = 1024L

  def writeDataEntryList(dataEntryList: List[DataEntry[_]], output: ByteArrayDataOutput): Unit =
    BinarySerializer.writeShortIterable(dataEntryList, dataEntryWrite, output)

  private def dataEntryWrite(value: DataEntry[_], output: ByteArrayDataOutput): Unit =
    output.write(toBytes(value))
}
