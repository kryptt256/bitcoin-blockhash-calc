use super::block_hash_util;

#[test]
fn test_calc_hash_block_490624() {
    let decoded_prev_hash = hex::decode("0000000000000000004239f2a01d8f579bc0dbb214d0f874ece5db587bee3457").expect("Error decoding previous block hash");
    let decoded_merkle_root = hex::decode("73effaecabddef72c9b6b738efb131c543370766b93d4cc15db995a9afb1a286").expect("Error decoding merkle root");
    let block_header = block_hash_util::BlockHeader {
        version: 536870912,
        hash_prev_block: decoded_prev_hash,
        hash_merkle_root: decoded_merkle_root,
        time: 1511062532,
        bits: 402713392,
        nonce: 3060038614
    };
    assert_eq!("000000000000000000d4c8b9d5388e42bf084e29546357c63cba8324ed4ec8bf", "");
}

#[test]
fn test_get_block_bytes() {
    let decoded_prev_hash = hex::decode("0000000000000000004239f2a01d8f579bc0dbb214d0f874ece5db587bee3457").expect("Error decoding previous block hash");
    let decoded_merkle_root = hex::decode("73effaecabddef72c9b6b738efb131c543370766b93d4cc15db995a9afb1a286").expect("Error decoding merkle root");
    let block_header = block_hash_util::BlockHeader {
        version: 536870912,
        hash_prev_block: decoded_prev_hash,
        hash_merkle_root: decoded_merkle_root,
        time: 1511062532,
        bits: 402713392,
        nonce: 3060038614
    };
    assert_eq!(block_hash_util::get_block_bytes(block_header), vec![0, 0, 0, 0, 236, 250, 239, 115, 4, 252, 16, 90, 48, 235, 0, 24, 214, 123, 100, 182]);
}

#[test]
fn get_sha256_blockhash() {
    let decoded_prev_hash = hex::decode("0000000000000000004239f2a01d8f579bc0dbb214d0f874ece5db587bee3457").expect("Error decoding previous block hash");
    let decoded_merkle_root = hex::decode("73effaecabddef72c9b6b738efb131c543370766b93d4cc15db995a9afb1a286").expect("Error decoding merkle root");
    let block_header = block_hash_util::BlockHeader {
        version: 536870912,
        hash_prev_block: decoded_prev_hash,
        hash_merkle_root: decoded_merkle_root,
        time: 1511062532,
        bits: 402713392,
        nonce: 3060038614
    };

    let block_bytes = block_hash_util::get_block_bytes(block_header);
    let digest = block_hash_util::get_sha256_blockhash(block_bytes);
    let block_hash = digest.as_ref();
    assert_eq!(hex::encode(block_hash), "342ce0611c8e31ad53b12a5d164df7aa6d4c861862c0578aaef7eb13607b3929");
}