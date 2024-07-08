use methods::METHOD_ID;

fn main() {
    let receipt = verify::deserialize_receipt("receipt.bin");
    receipt.verify(METHOD_ID).unwrap();
}
