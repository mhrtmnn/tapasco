/*
 *
 * Copyright (c) 2014-2020 Embedded Systems and Applications, TU Darmstadt.
 *
 * This file is part of TaPaSCo
 * (see https://github.com/esa-tu-darmstadt/tapasco).
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 */
/**
  * @file KernelTest.scala
  * @brief Unit tests for Kernel description file.
  * @authors J. Korinth, TU Darmstadt (jk@esa.cs.tu-darmstadt.de)
  **/
package tapasco.base

import org.scalatest._
import tapasco.TaPaSCoSpec
import tapasco.base.Kernel.PassingConvention._
import tapasco.base.json._

class KernelSpec extends TaPaSCoSpec with Matchers {

  val correctKernel = jsonPath.resolve("kernel/correctKernel")

  /**
    * Defines the required Reads[Kernel] as an implicit.
    */
  implicit val kernelReads = validatingKernelReads(correctKernel)

  "A missing Kernel file" should "throw an exception" in {
    assert(Kernel.from(correctKernel.resolve("missing.json")).isLeft)
  }

  "A correct Kernel file" should "be parsed to Right(Kernel)" in {
    assert(Kernel.from(correctKernel.resolve("correct-kernel.json")).isRight)
  }

  "A correct Kernel file" should "be parsed correctly" in {
    val oc = Kernel.from(correctKernel.resolve("correct-kernel.json"))
    lazy val c = oc.right.get
    assert(oc.isRight)
    c.name should equal("sudoku")
    c.topFunction should equal("sudoku_solve")
    c.files should equal(Seq(correctKernel.resolve("src/Sudoku.cpp"), correctKernel.resolve("src/Sudoku_HLS.cpp")))
    c.testbenchFiles should equal(
      Seq(correctKernel.resolve("src/main.cpp"), correctKernel.resolve("hard_sudoku.txt"), correctKernel.resolve("hard_sudoku_solution.txt")))
    c.compilerFlags should equal(Seq())
    c.testbenchCompilerFlags should equal(Seq("-lrt"))
    c.args.length should equal(1)
    c.args(0).name should equal("grid")
    c.args(0).passingConvention should equal(ByReference)
    c.otherDirectives should equal(Some(correctKernel.resolve("sudoku.dir")))
  }

  "A Kernel file with unknown entries" should "be parsed correctly" in {
    val oc = Kernel.from(correctKernel.resolve("correct-kernel.json"))
    lazy val c = oc.right.get
    assert(oc.isRight)
    c.name should equal("sudoku")
    c.topFunction should equal("sudoku_solve")
    c.files should equal(Seq(correctKernel.resolve("src/Sudoku.cpp"), correctKernel.resolve("src/Sudoku_HLS.cpp")))
    c.testbenchFiles should equal(Seq(correctKernel.resolve("src/main.cpp"), correctKernel.resolve("hard_sudoku.txt"),
      correctKernel.resolve("hard_sudoku_solution.txt")))
    c.compilerFlags should equal(Seq())
    c.testbenchCompilerFlags should equal(Seq("-lrt"))
    c.args.length should equal(1)
    c.args(0).name should equal("grid")
    c.args(0).passingConvention should equal(ByReference)
    c.otherDirectives should equal(Some(correctKernel.resolve("sudoku.dir")))
  }

  "An invalid Kernel file" should "not be parsed" in {
    val oc1 = Kernel.from(correctKernel.resolve("invalid-kernel1.json"))
    val oc2 = Kernel.from(correctKernel.resolve("invalid-kernel2.json"))
    val oc3 = Kernel.from(correctKernel.resolve("invalid-kernel3.json"))
    assert(oc1.isLeft)
    assert(oc2.isLeft)
    assert(oc3.isLeft)
  }
}
