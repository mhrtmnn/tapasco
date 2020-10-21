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
package tapasco.parser

import tapasco.PLATFORM_NUM_SLOTS
import tapasco.parser.FormatObject._

import scala.util.Properties.{lineSeparator => NL}

object Usage {
  private final val logger = tapasco.Logging.logger(getClass)
  private final val BINDENT = 74

  def apply()(implicit fmt: Formatter[String] = StringFormatter): String = fmt(usage)

  def topic(topic: Option[String])(implicit fmt: Formatter[String] = StringFormatter) {
    logger.info(fmt(helpTopics.get((topic getOrElse "usage" toLowerCase)).map(_.apply()) getOrElse usage))
    System.exit(0)
  }

  private final val helpTopics: Map[String, () => FormatObject] = Map(
    "jobsfile" -> jobsFile _,
    "configfile" -> configFile _,
    "globals" -> globals _,
    "bulkimport" -> bulkimport _,
    "compose" -> compose _,
    "corestats" -> corestats _,
    "import" -> importzip _,
    "hls" -> hls _,
    "explore" -> explore _,
    "composition" -> composition _,
    "features" -> features _,
    "topics" -> topics _,
    "manpage" -> manpage _
  )

  private def topics() = helpTopics.keys.toSeq.sorted
    .filterNot(_ equals "manpage") mkString ", "

  private def configFile() = Section("Config Files",
    """
A config file is a Json (http://json.org) file consisting of a single
configuration object. A configuration object can contain all parameters
available on the command line, including the jobs, which are represented
by an array of Job elements (same format as --jobsFile, see --help jobsFile
for more info).
    """.replace(NL, "") &
      "" &
      "You can generate an empty configuration via `tapasco -n config.json`.")

  private def jobsFile() = Section("Jobs Files",
    """
Jobs files are Json (http://json.org) files consisting of an array of Job
definitions. See $TAPASCO_HOME/json-examples/jobs/Jobs.json for an example
containing one instance of each job. Alternatively, generate an empty
configuration via `tapasco -n config.json`.
  """.replace(NL, ""))

  private def globals() = Section("Global Options",
    Arg("-v | --verbose [MODE]", "Verbose mode, log outputs of subprocesses;" ~
      "optional MODE is a quoted string selecting the output mode" &
      "(default: 'verbose')") &
      Arg("-n | --dryRun FILE", "Dry run, do not execute, only dump Json into FILE.") &
      Arg("--archDir PATH", "Base directory for architecture descriptions") &
      Arg("--compositionDir PATH", "Output base directory for Compose jobs") &
      Arg("--coreDir PATH", "Output base directory for HLS jobs, synthesized cores") &
      Arg("--kernelDir PATH", "Base directory for kernel descriptions (HLS)") &
      Arg("--platformDir PATH", "Base directory for platform descriptions") &
      Arg("--logFile FILE", "Path to output log file") &
      Arg("--configFile FILE", "Path to Json file with Configuration") &
      Arg("--jobsFile FILE", "Path to Json file with Jobs array") &
      Arg("--slurm TEMPLATE", "Activate SLURM cluster execution." ~
        "TEMPLATE describes a remote SLURM node, use 'local' for local execution (requires sbatch).") &
      Arg("--parallel", "Execute all jobs in parallel (careful!)") &
      Arg("--maxThreads NUM", "Limit internal parallelism of tasks (e.g., Vivado)" ~
        "to the given number of threads.") &
      Arg("--hlsTimeOut NUM", "Limit runtime of Vivado HLS to" ~
        "the given number of seconds.") &
      Arg("--maxTasks NUM", "Limit the parallelism of TaPaSCo to the given number." ~
        "This includes the main thread, so at most NUM-1 tasks are started in addition to the main thread."))

  private def composition() = Section("Composition Syntax",
    Block("A Composition specifies the number and kind of processing elements (PEs) that" ~
      "are present in the design. The basic command line syntax is as follows:") &
      "" &
      Arg("COMPOSITION", """'[' <KERNEL> x <COUNT> [, <KERNEL> x <COUNT>]* ']'""") &
      Indent(Arg("KERNEL", "name of kernel/core, a quoted/unquoted string") &
        Arg("COUNT", s"number of instances (0 < x <= ${PLATFORM_NUM_SLOTS})")) &
      "" &
      Arg("Examples:", "[ precision_counter x 128 ]" & "[arrayupdate x 4, arraysum x 8]"))

  private def features() = Section("Features Syntax",
    Block("The hardware designs generated by TaPaSCo offer a great amount of flexibility" ~
      "using a dynamic plug-in interface; a plug-in can extend or modify the" ~
      "resulting design. By default, most plug-ins are disabled and must be activated" ~
      "by the user. This is done via a Feature specification: A Feature contains the" ~
      "configuration parameters for a plug-in. The basic command line syntax is as" ~
      "follows:") &
      "" &
      Arg("FEATURE", "<NAME> <BEGIN> [<KEYVALUE> [, <KEYVALUE>]*] <END>") &
      Indent(Arg("NAME", "any quoted or unquoted string") &
        Indent(Arg("BEGIN", "one of '[', '{' or '('") &
          Arg("END", "one of ']', '}, or ')'")) &
        Arg("KEYVALUE", "<KEY> <ASSIGN> <VALUE>") &
        Indent(Arg("KEY", "any quoted or unquoted string") &
          Arg("ASSIGN", "either '->', '=', ':=' or ':'") &
          Arg("VALUE", "any quoted or unquoted string"))) &
      "" &
      Arg("Examples:", "'OLED' [enabled -> true]" &
        """'LEDS' { enabled: true, inputs: "/system/LED_*" }""" &
        "'BlueDMA' (enabled = true)"))

  private def bulkimport() = Section("Bulk Import Job",
    Block("Bulk import jobs can import multiple IP-XACT core .zip files in row. The" ~
      "required import parameters are given a CSV file instead of manually via the" ~
      "command line.") &
      "" &
      Arg("Syntax", "bulkimport <CSV>") &
      Indent("where" &
        Arg("CSV", "Path to file in comma-separated values (CSV) format," ~
          "must contain the following header line and columns:") &
        "" &
        "Zip, ID, Description, Architecture, Platform, Avg Runtime (clock cycles)"))

  private def compose() = Section("Compose Job",
    Block("Generate a full-system bitstream from spec. Compose jobs are the core of" ~
      "TaPaSCo and comprise the construction of the system design, synthesis and" ~
      "place-and-route, as well as high-level synthesis of cores (if necessary).") &
      "" &
      "To generate a design, one needs to specify" &
      "" &
      Indent("1) the Composition (see below)" &
        "2) the Architecture (i.e., organization of PEs in the design)" &
        "3) the Platform (i.e., target hardware)" &
        "4) the target design frequency (operating frequency of the PEs)") &
      "" &
      Block("If Architecture or Platform selection is ommitted, TaPaSCo will build" ~
        "bitstreams for all available Architectures and Platforms in parallel." ~
        "Resource restrictions apply normally (e.g., won't launch more runs than" ~
        "CPUs available, etc.). The resulting projects and bitstreams can be found" ~
        "in the directory hierarchy below the currently configured Composition" ~
        "directory (see `tapasco -h globals`).") &
      "" &
      Arg("Syntax", "compose <COMPOSITION> @ <NUM> MHz [option]*") &
      Indent("where" &
        Arg("COMPOSITION", "Composition spec, see `tapasco -h composition`") &
        Arg("NUM", "target operating frequency for PEs in MHz")) &
      "" &
      "followed optionally by:" &
      Indent(Arg("-a | --architectures A+", "list of Architectures to compose for, e.g.," ~
        "-a axi4mm, myarch") &
        Arg("-p | --platforms P+", "list of Platforms to compose for, e.g.," ~
          "-p vc709, pynq") &
        Arg("--implementation NAME", "selects a tool for synthesis, place-and-route" &
          """default: "Vivado"""") &
        Arg("--features FEATURES", "configures Features, see `tapasco -h features`" &
          "syntax: FEATURE [, FEATURE]*") &
        Arg("--deleteProjects (true | false)?", "Spefify whether project files are deleted or kept" &
          """default: true""") &
        Arg("--debugMode NAME", "dry run, no composition is executed; modes:") &
        Indent(Arg("r", "generate random result values") &
          Arg("t", "generate only timing failures") &
          Arg("p", "generate only placer errors") &
          Arg("o", "generate only other errors")) &
        Arg("--effortLevel EFFORT", "set effort level for synthesis and PnR; levels:") &
        Indent(Arg("fastest", "lowest effort, minimal runtime") &
          Arg("fast", "slightly slower, but still short runtime") &
          Arg("normal", "default options") &
          Arg("optimal", "slower, get best QoR possible") &
          Arg("aggressive_performance", "maximal optimization for performance") &
          Arg("aggressive_area", "maximal optimization for area")) &
        Arg("--skipSynthesis", "For testing purposes, execute composition without final" ~
          "synthesis and bitstream generation")) &
      "" &
      s"NOTE: Currently the  total number of PEs must be <= ${PLATFORM_NUM_SLOTS}." &
      s"IMPORTANT: The maximum runtime of a compose job is limited is limited dependent on the platform.")

  private def corestats() = Section("Core Statistics Job",
    Block("Evaluation helper job that automatically gathres the out-of-context results" ~
      "for all Cores, Architectures and Platforms in one .csv file per Architecture" ~
      "and Platform combination. Useful for quick overviews over the available PEs.") &
      "" &
      Arg("Syntax", "corestats [option]*") &
      "" &
      "followed optionally by:" &
      Indent(Arg("-a | --architectures A+", "list of Architectures , e.g.," ~
        "-a axi4mm, myarch") &
        Arg("-p | --platforms P+", "list of Platforms , e.g.," ~
          "-p vc709, pynq") &
        Arg("--prefix STRING", "file names of generated .csv files will be of the" ~
          "format STRING<ARCH>@<PLATFORM>.csv")))

  private def importzip() = Section("Import Job",
    Block("TaPaSCo supports the use of High-Level Synthesis (HLS) tools (such as Xilinx" ~
      "Vivado HLS) for the synthesis of processing element hardware modules from" ~
      "C/C++ automatically (see `tapasco -h hls`). To make existing IP available as" ~
      "PEs in TaPaSCo, you can use the import command:") &
      "" &
      Arg("Syntax", "import <ZIP> as <ID> [option]*") &
      Indent("where" &
        Arg("ZIP", "path to .zip file containing an IP-XACT" ~
          "description (e.g., component.xml and Verilog/VHDL" ~
          "sources); can be generated, e.g., via Xilinx" ~
          "Vivado, menu Tools -> Create and package IP.") &
        Arg("ID", "any integer > 0; this ID is used to identify the" ~
          "PEs in the hardware and software layers of TaPaSCo" ~
          "Core with the same ID are considered to be" ~
          "alternative implementations of the same interface" ~
          "and should be exchangeable (see `tapasco -h" ~
          "explore`).")) &
      "" &
      "followed optionally by:" &
      Indent(Arg("-a | --architectures A+", "list of Architectures , e.g.," ~
        "-a axi4mm, myarch") &
        Arg("-p | --platforms P+", "list of Platforms , e.g.," ~
          "-p vc709, pynq") &
        Arg("--description TEXT", "any quoted or unquoted string containing" ~
          "additional information about the core") &
        Arg("--skipEvaluation", "do not perform out-of-context synthesis to get" ~
          "resource estimates, only import") &
        Arg("--averageClockCycles N", "any integer > 0; number of clock cycles in an" ~
          """"average" execution of the PE; used to estimate""" ~
          "the maximal throughput")))

  private def hls() = Section("High-Level Synthesis Job",
    Block("TaPaSCo facilitates rapid prototyping for FPGA accelerators by directly" ~
      "supporting hardware written in C/C++ via HLS. The hls job is used to trigger" ~
      "the HLS tool and synthesize hardware for a given Architecture and Platform.") &
      "" &
      Block("If Architecture or Platform selection is ommitted, TaPaSCo will build" ~
        "cores for all available Architectures and Platforms in parallel." ~
        "Resource restrictions apply normally (e.g., won't launch more runs than" ~
        "CPUs available, etc.). The resulting cores can be found in the directory" ~
        "hierarchy below the currently configured Core directory" ~
        "(see `tapasco -h globals`).") &
      "" &
      Arg("Syntax", "hls <KERNELS> [option]*") &
      Indent("where" &
        Arg("KERNELS", "all | <KERNEL> [, <KERNEL]*" &
          "where KERNEL is a kernel name as quoted or" ~
            "unquoted string; special target 'all' builds all" ~
            "available kernels.")) &
      "" &
      "followed optionally by:" &
      Indent(Arg("-a | --architectures A+", "list of Architectures , e.g.," ~
        "-a axi4mm, myarch") &
        Arg("-p | --platforms P+", "list of Platforms , e.g.," ~
          "-p vc709, pynq") &
        Arg("--implementation NAME", "selects a HLS tool by name" &
          """default: "VivadoHLS"""") &
        Arg("--skipEvaluation", "import the HLS result without performing" ~
          "an out-of-context synthesis to get resource estimates")) &
      "" &
      Section("Note",
        Block("All HLS kernels are located in the directories below the currently" ~
          "configured Kernel directory (see `tapasco -h globals`). Each kernel" ~
          "requires a description in a simple Json format, examples can be found" ~
          "in $TAPASCO_HOME/kernel.", BINDENT)))

  // scalastyle:off method.length
  private def explore() = Section("Design Space Exploration Job",
    Block("Even simple hardware designs often require a surprisingly high number of" ~
      "design choices. It is difficult to estimate the impact of each choice on" ~
      "the total result. TaPaSCo supports the designer by offering an automated" ~
      "Design Space Exploration (DSE): TaPaSCo designs can primarily be varied in" ~
      "three dimensions:") &
      "" &
      Indent(Arg("1) Area / Utilization", "primarily determined by the number of PEs.") &
        Arg("2) Target Frequency", "chosen operating frequency") &
        Arg("3) Alternatives", "a choice of alternative hardware implementations" ~
          "for a kernel (identified by their ID, see" ~
          "`tapasco -h import`)")) &
      "" &
      Block("TaPaSCo's DSE mode can automatically explore this design space using a user-" /
        "selectable performance heuristic. The default heuristic approximates the" ~
          "maximal job throughput of the system: Current operating frequency and the" ~
          "average clock cycles per job execution of each PE in the design are" ~
          "extrapolated with the instance numbers to yield an upper bound on the total" ~
          "job throughput of the system. This number is used as a relative \"fitness\"" ~
          "indicator for the comparison of different Compositions. The design space can" ~
          "be linearized by descending value for each element.") &
      "" &
      Block("TaPaSCo explores the design space by batches: Each batch consists of a" ~
        "configurable number of design space elements (i.e., Composition + frequency" ~
        "pairs); all elements are run in parallel via 'compose'. The successful" ~
        "element with the highest heuristic value is returned.") &
      "" &
      "In case of errors, three cases must be distinguished:" &
      "" &
      Indent(Block(I("Placer errors") ~ "affect all design space elements with the same or a higher" ~
        "number of PEs; none of them will be placeable and they will therefore" ~
        "be pruned from the design space.") &
        "" &
        Block(I("Timing failures") ~ "affect only the given element, but generate a feedback" ~
          "element: A new design space element is generated for the same" ~
          "Composition, but with a lower target frequency. The frequency is" ~
          "reduced by 5 MHz for the next run.") &
        "" &
        Block(I("Other errors:") ~ "This encompasses all other errors, e.g., missing licenses," ~
          "system crashes, out-of-memory problems, etc.")) &
      "" &
      Block("TaPaSCo can run explorations in any combination of the three dimensions. To" ~
        "get a better idea of each dimension, you can use `itapasco` to configure DSE" ~
        "and get a preview of each active dimension.") &
      "" &
      Arg("Syntax", "explore <COMPOSITION> [<FREQ>] in <DIMS> [option]*") &
      Indent(Arg("COMPOSITION", "Composition spec, see `tapasco -h composition`") &
        Arg("FREQ", "'@' <NUM> [MHz]" &
          "initial design frequency in MHz, optional") &
        Arg("DIMS", "list of active dimensions, e.g., area, frequency, alternatives")) &
      "" &
      "followed optionally by:" &
      Indent(Arg("-a | --architectures A+", "list of Architectures , e.g.," ~
        "-a axi4mm, myarch") &
        Arg("-p | --platforms P+", "list of Platforms , e.g.," ~
          "-p vc709, pynq") &
        Arg("--basePath PATH", "output base path for DSE results, including" ~
          "config files, projects and bitstreams" &
          "default: DSE_<CURRENT DATE>") &
        Arg("--batchSize NUM", "number of elements in each batch" &
          "default: number of CPUs") &
        Arg("--debugMode NAME", "dry run, no compositions are executed, see" ~
          "`tapasco -h compose`") &
        Arg("--deleteProjects ( true | false )?", "Spefify whether project files are deleted or kept" &
          """default: true""") &
        Arg("--features FEATURES", "configures Features, see `tapasco -h features`" &
          "syntax: FEATURE [, FEATURE]*") &
        Arg("--heuristic NAME", "select heuristic function" &
          "default: 'job throughput'") &
        Arg("--implementation NAME", "selects a tool for synthesis, place-and-route" &
          "default: \"Vivado\"")) &
      "" &
      Section("Note",
        Block("All HLS kernels are located in the directories below the currently" ~
          "configured Kernel directory (see `tapasco -h globals`). Each kernel" ~
          "requires a description in a simple Json format, examples can be found" ~
          "in $TAPASCO_HOME/kernel.", BINDENT)) &
      s"IMPORTANT: The maximum runtime of a single design-space entity run is limited dependent on the platform.")

  // scalastyle:on method.length

  private def shortJobs() = Section("Jobs",
    Arg("", "(see `tapasco -h <job>` for details)") &
      Arg("bulkimport", "Bulk import of IP-XACT core .zips.") &
      Arg("compose", "Generate bitstream for a single Composition.") &
      Arg("corestats", "Generate a .csv file with synth data for Cores.") &
      Arg("explore", "Perform a design space exploration.") &
      Arg("hls", "Perform high-level synthesis for PEs.") &
      Arg("import", "Import a single IP-XACT core .zip."))

  private def shortTopics() = Section("Help Topics", Block(helpTopics.keys.toSeq.sorted.mkString(", ")))

  private def overview() =
    Header("tapasco", 1, "2018-01-22", "2018.1") &
      Synopsis("    tapasco [global option]* [job]*" &
        "or: tapasco -h | --help [TOPIC]" &
        "")

  private def usage()(implicit fmt: Formatter[String] = StringFormatter) = Seq(
    overview(),
    globals,
    shortJobs,
    shortTopics
  ) map (fmt.apply _) mkString

  private def manpage() = {
    implicit val fmt = ManPageFormatter
    val exclude = Seq("topics", "manpage", "globals")
    val fos = Seq(overview, globals) ++
      helpTopics.keys.toSeq.sorted.filterNot(exclude.contains(_)).map(helpTopics(_)())
    fos map (fmt.apply _) mkString NL
  }
}
