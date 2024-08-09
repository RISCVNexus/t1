// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2022 Jiuyang Liu <liu@jiuyang.me>

package org.chipsalliance.t1.rtl.decoder.attribute

import org.chipsalliance.t1.rtl.decoder.T1DecodePattern

trait ZvkUOPType extends Uop
object zvkUop0 extends ZvkUOPType // 
object zvkUop1 extends ZvkUOPType // 
object zvkUop2 extends ZvkUOPType // 
object zvkUop3 extends ZvkUOPType // 
object zvkUop4 extends ZvkUOPType // 
object zvkUop5 extends ZvkUOPType // 
object zvkUop6 extends ZvkUOPType // 
object zvkUop7 extends ZvkUOPType // 
object zvkUop8 extends ZvkUOPType // 

object ZvkUOP {
  def apply(t1DecodePattern: T1DecodePattern): Uop = {
    Seq(
      t0 _ -> zvkUop0,
      t1 _ -> zvkUop1,
      t2 _ -> zvkUop2,
      t3 _ -> zvkUop3,
      t4 _ -> zvkUop4,
      t5 _ -> zvkUop5,
      t6 _ -> zvkUop6,
      t7 _ -> zvkUop7,
      t8 _ -> zvkUop8,
    ).collectFirst {
      case (fn, tpe) if fn(t1DecodePattern) => tpe
    }.getOrElse(UopDC)
  }
  def t0(t1DecodePattern: T1DecodePattern): Boolean = {
    val allMatched: Seq[String] = Seq(
    )
    allMatched.contains(t1DecodePattern.instruction.name)
  }
  def t1(t1DecodePattern: T1DecodePattern): Boolean = {
    val allMatched: Seq[String] = Seq(
    )
    allMatched.contains(t1DecodePattern.instruction.name)
  }
  def t2(t1DecodePattern: T1DecodePattern): Boolean = {
    val allMatched: Seq[String] = Seq(
    )
    allMatched.contains(t1DecodePattern.instruction.name)
  }
  def t3(t1DecodePattern: T1DecodePattern): Boolean = {
    val allMatched: Seq[String] = Seq(
    )
    allMatched.contains(t1DecodePattern.instruction.name)
  }
  def t4(t1DecodePattern: T1DecodePattern): Boolean = {
    val allMatched: Seq[String] = Seq(
    )
    allMatched.contains(t1DecodePattern.instruction.name)
  }
  def t5(t1DecodePattern: T1DecodePattern): Boolean = {
    val allMatched: Seq[String] = Seq(
    )
    allMatched.contains(t1DecodePattern.instruction.name)
  }
  def t6(t1DecodePattern: T1DecodePattern): Boolean = {
    val allMatched: Seq[String] = Seq(
    )
    allMatched.contains(t1DecodePattern.instruction.name)
  }
  def t7(t1DecodePattern: T1DecodePattern): Boolean = {
    val allMatched: Seq[String] = Seq(
    )
    allMatched.contains(t1DecodePattern.instruction.name)
  }
  def t8(t1DecodePattern: T1DecodePattern): Boolean = {
    val allMatched: Seq[String] = Seq(
    )
    allMatched.contains(t1DecodePattern.instruction.name)
  }
}
