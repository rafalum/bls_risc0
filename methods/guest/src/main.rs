use risc0_zkvm::guest::env;

extern crate blst;
use blst::min_pk::*;

fn main() {

    // read the number of public keys
    let number_keys = env::read();

    // read the public keys
    let mut public_keys = vec![0u8; 96 * number_keys];
    env::read_slice(&mut public_keys);

    // deserialize the public keys
    let mut pks = Vec::with_capacity(number_keys);
    for pk in public_keys.chunks(96) {  
        pks.push(PublicKey::deserialize(&pk).unwrap());
    }
    let pks_refs: Vec<&PublicKey> = pks.iter().collect();

    let agg = match AggregatePublicKey::aggregate(&pks_refs, false){
        Ok(agg_pks) => agg_pks,
        Err(err) => panic!("aggregate failure: {:?}", err),
    };
    let aggregate_public_key = agg.to_public_key();

    // write public output to the journal
    env::commit_slice(&aggregate_public_key.serialize());
}