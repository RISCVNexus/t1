// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2024 Jiuyang Liu <liu@jiuyang.me>
package org.chipsalliance.t1.elaborator.rocketv

import chisel3.experimental.util.SerializableModuleElaborator
import mainargs._
import org.chipsalliance.rocketv.{ALU, ALUParameter}

object ALU extends SerializableModuleElaborator {
  implicit object PathRead extends TokensReader.Simple[os.Path] {
    def shortName = "path"
    def read(strs: Seq[String]) = Right(os.Path(strs.head, os.pwd))
  }

  val className: String = getClass.getSimpleName.replace("$", "")
  type D = ALU
  type P = ALUParameter
  type M = ALUParameterMain

  @main
  case class ALUParameterMain(
    @arg(name = "xLen") xLen: Int) {
    def convert: P = ALUParameter(xLen)
  }

  implicit def ALUParameterMainParser: ParserForClass[M] = ParserForClass[M]

  @main
  def config(@arg(name = "parameter") parameter: M) =
    os.write.over(os.pwd / s"${className}.json", configImpl(parameter.convert))

  @main
  def design(@arg(name = "parameter") parameter: os.Path) = {
    val (firrtl, annos) = designImpl[D, P](os.read.stream(parameter))
    os.write.over(os.pwd / s"$className.fir", firrtl)
    os.write.over(os.pwd / s"$className.json", annos)
  }

  def main(args: Array[String]): Unit = ParserForMethods(this).runOrExit(args)
}
