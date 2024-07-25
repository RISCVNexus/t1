// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2022 Jiuyang Liu <liu@jiuyang.me>

import mill._
import mill.scalalib._
import mill.scalalib.publish._
import mill.scalalib.scalafmt._

trait ScriptModule extends ScalaModule {
  val scala3 = "3.3.3"
  val mainargs = ivy"com.lihaoyi::mainargs:0.5.0"
  val oslib = ivy"com.lihaoyi::os-lib:0.10.0"
  val upickle = ivy"com.lihaoyi::upickle:3.3.1"

  def scalaVersion = scala3
  def scalacOptions = Seq("-new-syntax")
  override def ivyDeps = Agg(mainargs, oslib, upickle)
}

object emu extends ScriptModule {

}

object ci extends ScriptModule {

}
