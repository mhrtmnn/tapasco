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
  * @file FeatureTclPrinter.scala
  * @brief Generates Tcl commands to add feature to a Tcl dict.
  * @authors J. Korinth, TU Darmstadt (jk@esa.cs.tu-darmstadt.de)
  **/
package tapasco.base.tcl

import tapasco.base.Feature

import scala.util.Properties.{lineSeparator => NL}

class FeatureTclPrinter(prefix: String = "") {
  private val pre = s"dict set ${prefix}features"

  /** GenerateTcl commands to add feature to a Tcl dict.
    *
    * @param f Feature to add.
    * @return String containing Tcls commands to write f into
    *         a dict called <prefix>features.
    */
  def toTcl(f: Feature): String = f.props.value map {
    case (name, value) => s"$pre ${f.name} $name ${value.toTCL}"
  } mkString NL

  def toTcl(fs: Seq[Feature]): String = fs.map(toTcl).mkString(NL)
}
