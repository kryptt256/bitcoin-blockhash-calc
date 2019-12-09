extern crate hex;

use super::block_hash_util::BlockHeader;

#[test]
fn test_calc_hash_block_490624() {
    let decoded_prev_hash = hex::decode("0000000000000000004239f2a01d8f579bc0dbb214d0f874ece5db587bee3457").expect("Error decoding previous block hash");
    let decoded_merkle_root = hex::decode("73effaecabddef72c9b6b738efb131c543370766b93d4cc15db995a9afb1a286").expect("Error decoding merkle root");
    let block_header = BlockHeader {
        version: 536870912,
        hash_prev_block: decoded_prev_hash,
        hash_merkle_root: decoded_merkle_root,
        time: 1511062532,
        bits: 402713392,
        nonce: 3060038614
    };
    assert_eq!("000000000000000000d4c8b9d5388e42bf084e29546357c63cba8324ed4ec8bf", "");
}