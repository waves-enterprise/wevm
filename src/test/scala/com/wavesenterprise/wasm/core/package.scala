package com.wavesenterprise.wasm

import com.google.common.io.{ByteArrayDataOutput, ByteStreams}
import com.wavesenterprise.serialization.BinarySerializer
import com.wavesenterprise.state.DataEntry
import com.wavesenterprise.transaction.docker.ContractTransactionEntryOps.{parse, toBytes}

package object core {
  def parseDataEntry(bytes: Array[Byte]): DataEntry[_] =
    parse(bytes, 0)._1

  def serializeDataEntryList(list: List[DataEntry[_]]): Array[Byte] = {
    val bytes: ByteArrayDataOutput = ByteStreams.newDataOutput()
    writeDataEntryList(list, bytes)
    bytes.toByteArray()
  }

  private def writeDataEntryList(dataEntryList: List[DataEntry[_]], output: ByteArrayDataOutput): Unit =
    BinarySerializer.writeShortIterable(dataEntryList, dataEntryWrite, output)

  private def dataEntryWrite(value: DataEntry[_], output: ByteArrayDataOutput): Unit =
    output.write(toBytes(value))
}
