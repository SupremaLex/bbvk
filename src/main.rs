use noir_rs::barretenberg::{prove::get_verification, srs::setup_srs};

fn main() {}

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

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    #[derive(serde::Deserialize)]
    struct Circuit {
        bytecode: String,
    }

    #[test]
    fn test_get_verification_key() {
        let trusted_setup_path = "assets/ultraPlonkTrustedSetup.dat";

        let mut circuit_json_file = File::open("assets/testCircuit.json").unwrap();

        let mut circuit_json = String::new();
        circuit_json_file.read_to_string(&mut circuit_json).unwrap();

        let circuit: Circuit = serde_json::from_str(&circuit_json).unwrap();

        super::setup_srs_swift(circuit.bytecode.clone(), Some(trusted_setup_path), false).unwrap();

        let vk = super::get_verification_key_swift(circuit.bytecode).unwrap();

        println!("Verification Key: {:?}", vk);
    }
}
