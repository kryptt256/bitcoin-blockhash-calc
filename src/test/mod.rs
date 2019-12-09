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
    let block_bytes = block_hash_util::get_block_bytes(block_header);
    let digest = block_hash_util::get_sha256_blockhash(block_bytes);
    let block_hash = digest.as_ref();
    let mut bitcoin_block_hash = Vec::from(block_hash);
    bitcoin_block_hash.reverse();
    assert_eq!("dbfaba4d71021c1b651474dfe386214f5de4c123191926671e6096aa23f22298", hex::encode(bitcoin_block_hash));
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
    assert_eq!(block_hash_util::get_block_bytes(block_header), vec![0, 0, 0, 32, 87, 52, 238, 123, 88, 219, 229, 236, 116, 248, 208, 20, 178, 219, 192, 155, 87, 143, 29, 160, 242, 57, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 134, 162, 177, 175, 169, 149, 185, 93, 193, 76, 61, 185, 102, 7, 55, 67, 197, 49, 177, 239, 56, 183, 182, 201, 114, 239, 221, 171, 236, 250, 239, 115, 4, 252, 16, 90, 48, 235, 0, 24, 214, 123, 100, 182]);
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
    assert_eq!(hex::encode(block_hash), "9822f223aa96601e6726191923c1e45d4f2186e3df7414651b1c02714dbafadb");
}