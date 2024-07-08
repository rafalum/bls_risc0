use risc0_zkvm::Receipt;
use bincode;

use std::fs::File;
use std::io::{Read, BufReader};

pub fn deserialize_receipt(file_path: &str) -> Receipt {

    // Reading and deserializing the receipt from the file
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    // Read the entire file into the buffer
    reader.read_to_end(&mut buffer).unwrap();

    // Deserialize the Vec<u32> back into a Receipt
    let receipt: Receipt = bincode::deserialize(&buffer).unwrap();

    return receipt;
}