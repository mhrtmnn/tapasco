#[macro_use]
extern crate log;
#[macro_use]
extern crate common_failures;
#[macro_use]
extern crate failure;
extern crate env_logger;
extern crate hex;
extern crate serde;
extern crate serde_json;

use prost::Message;
use std::u64;

use common_failures::prelude::*;

use serde::Deserialize;

use std::fs;
use std::fs::File;
use std::path::Path;

use std::io::BufReader;

use clap::{App, AppSettings, Arg};

pub mod status {
    include!(concat!(env!("OUT_DIR"), "/tapasco.status.rs"));
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Composition {
    Type: String,
    SlotId: u64,
    Kernel: u64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Version {
    Software: String,
    Year: u64,
    Release: u64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Clocks {
    Domain: String,
    Frequency: u64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Component {
    Name: String,
    Address: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct ComponentAddresses {
    Base: String,
    Components: Vec<Component>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct PEAddresses {
    Base: String,
    Offsets: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct BaseAddresses {
    Architecture: PEAddresses,
    Platform: ComponentAddresses,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Design {
    Composition: Vec<Composition>,
    Timestamp: u64,
    Versions: Vec<Version>,
    Clocks: Vec<Clocks>,
    BaseAddresses: BaseAddresses,
}

#[derive(Debug, Fail)]
pub enum JSONToStatusError {
    #[fail(display = "Invalid json format for input file {}", filename)]
    JSONFormatError {
        #[fail(cause)]
        err: serde_json::Error,
        filename: String,
    },

    #[fail(display = "Could not convert HEX string to u64")]
    HexToIntError {
        #[fail(cause)]
        err: std::num::ParseIntError,
    },

    #[fail(
        display = "Missing input argument. This error should've been caught be the CLI parser."
    )]
    MissingInput,

    #[fail(
        display = "Missing output argument. This error should've been caught be the CLI parser."
    )]
    MissingOutput,

    #[fail(display = "Found local memory but no preceeding PE.")]
    MemoryWithoutPE,
}

impl From<std::num::ParseIntError> for JSONToStatusError {
    fn from(err: std::num::ParseIntError) -> JSONToStatusError {
        JSONToStatusError::HexToIntError { err: err }
    }
}

impl From<serde_json::Error> for JSONToStatusError {
    fn from(err: serde_json::Error) -> JSONToStatusError {
        JSONToStatusError::JSONFormatError {
            err: err,
            filename: "UNKNOWN".to_string(),
        }
    }
}

impl JSONToStatusError {
    fn from_filename(filename: String) -> impl Fn(serde_json::Error) -> JSONToStatusError {
        move |x| JSONToStatusError::JSONFormatError {
            err: x,
            filename: filename.clone(),
        }
    }
}

fn from_hex_str(s: &String) -> Result<u64> {
    u64::from_str_radix(s.trim_start_matches("0x"), 16)
        .map_err(JSONToStatusError::from)
        .map_err(failure::Error::from)
}

fn write_mem_file(filename: &Path, data: &[u8]) -> Result<()> {
    info!("Generating hex representation of flatbuffer");
    let hex_data: Vec<char> = hex::encode(data).chars().collect();
    let mut init_vec = String::new();
    for c in hex_data.chunks(2) {
        let cstr = c.iter().cloned().collect::<String>();
        if init_vec.is_empty() {
            init_vec = format!("{}", cstr);
        } else {
            init_vec = format!(
                "{},
                {}",
                init_vec, cstr
            );
        }
    }

    let coe_content = format!(
        "memory_initialization_radix=16;
    memory_initialization_vector=
    {};
    ",
        init_vec
    );
    trace!("Generated {}", coe_content);
    info!("Writing to file {:?}", filename);
    fs::write(filename, coe_content).io_read_context(filename)?;

    Ok(())
}

fn run() -> Result<()> {
    env_logger::init();

    let matches = App::new("json_to_status")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1")
        .about("Converts a JSON file describing a TaPaSCo Design into a flatbuffer binary format readable by Vivado as MEM file.")
        .arg(
            Arg::with_name("INPUT")
                .help("JSON file generated from TaPaSCo design flow")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Hex encoded file for use in BRAM initialization")
                .takes_value(true)
                .required(true),
        )
        .arg(Arg::with_name("binary").long("--binary").short("-b").help("Output binary representation of ProtoBuf as well."))
        .get_matches();

    let input_file_name = matches
        .value_of("INPUT")
        .ok_or_else(|| JSONToStatusError::MissingInput)?;
    info!("Opening input JSON file {}", input_file_name);
    let json_input = File::open(input_file_name)?;
    let json_reader = BufReader::new(json_input);
    info!("Parsing JSON file {}", input_file_name);

    let json: Design = serde_json::from_reader(json_reader).map_err(
        JSONToStatusError::from_filename(input_file_name.to_string()),
    )?;

    info!("Successfully parsed JSON file {}", input_file_name);
    trace!("{} => {:#?}", input_file_name, json);

    info!("Starting to build the binary representation");

    let arch_base = from_hex_str(&json.BaseAddresses.Architecture.Base)?;

    let platform_base = from_hex_str(&json.BaseAddresses.Platform.Base)?;

    info!(
        "Architecture start: 0x{:X}, Platform start: 0x{:X}",
        arch_base, platform_base
    );

    let mut pes: Vec<status::Pe> = Vec::new();
    for pe in json.Composition {
        let addr = from_hex_str(&json.BaseAddresses.Architecture.Offsets[pe.SlotId as usize])?;
        if pe.Type == "Memory" {
            let last = pes
                .last_mut()
                .ok_or_else(|| JSONToStatusError::MemoryWithoutPE)?;
            last.local_memory = addr
        } else {
            pes.push(status::Pe {
                name: "UNNAMED KERNEL".to_string(),
                id: pe.Kernel as u32,
                offset: addr,
                local_memory: 0,
            });
        }
    }

    let clocks: Vec<_> = json
        .Clocks
        .iter()
        .map(|x| status::Clock {
            name: x.Domain.clone(),
            frequency_mhz: x.Frequency as u32,
        })
        .collect();

    let platforms: Vec<_> = json
        .BaseAddresses
        .Platform
        .Components
        .iter()
        .map(|x| status::Platform {
            name: x.Name.clone(),
            offset: from_hex_str(&x.Address).unwrap(),
        })
        .collect();

    let versions: Vec<_> = json
        .Versions
        .iter()
        .map(|x| status::Version {
            software: x.Software.clone(),
            year: x.Year as u32,
            release: x.Release as u32,
        })
        .collect();

    let status = status::Status {
        arch_base: arch_base,
        platform_base: platform_base,
        timestamp: json.Timestamp,
        pe: pes,
        platform: platforms,
        clocks: clocks,
        versions: versions,
    };

    let mut buf: Vec<u8> = Vec::new();
    status.encode(&mut buf)?;

    info!(
        "Successfully generated binary protobuf representation: {} bytes",
        status.encoded_len()
    );
    let output_file_name = matches
        .value_of("OUTPUT")
        .ok_or_else(|| JSONToStatusError::MissingOutput)?;

    if matches.is_present("binary") {
        let ofn = format!("{}.bin", output_file_name);
        let ofp = Path::new(&ofn);
        info!("Outputting binary as well to {}", ofn);
        fs::write(ofp, &buf).io_read_context(ofp)?;
    }

    let output_path = Path::new(output_file_name);
    write_mem_file(output_path, &buf)
}

quick_main!(run);
