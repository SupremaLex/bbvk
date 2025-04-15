use std::{fs::File, io::Read};

use clap::Parser;
use serde::Deserialize;

use noir_rs::barretenberg::{prove::get_verification, srs::setup_srs};

/// Plonk vetification key generator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the trusted setup file
    #[arg(short, long)]
    setup_path: String,

    /// Path to the circuit bytecode file
    #[arg(short, long)]
    circuit_path: String,

    /// Path to output the verification key
    #[arg(short, long)]
    output_path: String,
}

fn main() {
    let args = Args::parse();

    // Read the circuit bytecode from the file
    let mut circuit_json_file = File::open(args.circuit_path).unwrap();
    let mut circuit_json = String::new();
    circuit_json_file.read_to_string(&mut circuit_json).unwrap();

    let circuit: Circuit = serde_json::from_str(&circuit_json).unwrap();

    // Setup SRS
    setup_srs(&circuit.bytecode, Some(&args.setup_path), false).unwrap();

    // Get the verification key
    let verification_key = get_verification(&circuit.bytecode).unwrap();

    // Write the verification key to the output file
    std::fs::write(args.output_path, hex::encode(verification_key)).unwrap();
}

#[derive(Deserialize)]
struct Circuit {
    bytecode: String,
}

pub fn get_verification_key_swift(circuit_bytecode: String) -> Option<String> {
    match get_verification(&circuit_bytecode) {
        Ok(vkey) => Some(hex::encode(vkey)),
        Err(_) => None,
    }
}

pub fn setup_srs_swift(
    circuit_bytecode: String,
    srs_path: Option<&str>,
    recursive: bool,
) -> Option<u32> {
    setup_srs(&circuit_bytecode, srs_path, recursive).ok()
}
