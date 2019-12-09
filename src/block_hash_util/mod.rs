extern crate ring;

use ring::digest::{Context, Digest, SHA256};

pub struct BlockHeader {
    pub version: u32,
    pub hash_prev_block: Vec<u8>,
    pub hash_merkle_root: Vec<u8>,
    pub time: u32,
    pub bits: u32,
    pub nonce: u32
}

pub fn get_block_bytes(block_header: BlockHeader) -> Vec<u8> {
    let mut prev_block = block_header.hash_prev_block;
    prev_block.reverse();
    let mut merkle_root = block_header.hash_merkle_root;
    merkle_root.reverse();

    let header_last = vec![block_header.time, block_header.bits, block_header.nonce]; 
    concat_block_header(block_header.version, &mut prev_block, &mut merkle_root, header_last)
}

fn concat_block_header(version: u32, prev_block: &mut Vec<u8>, merkle_root: &mut Vec<u8>, header_last: Vec<u32>) -> Vec<u8> {
    let mut wtr: Vec<u8> = vec![];
    wtr.extend_from_slice(&version.to_le_bytes());
    wtr.extend_from_slice(&prev_block);
    wtr.extend_from_slice(&merkle_root);
    header_last.iter().for_each(|number| wtr.extend_from_slice(&(*number).to_le_bytes()));
    wtr
}

pub fn get_sha256_blockhash(bytes: Vec<u8>) -> Digest {
    let byte_slice = bytes.as_slice();
    let hash1 = get_digest_hash_ref(byte_slice);
    let hash2 = get_digest_hash_ref(hash1.as_ref());
    hash2
}

fn get_digest_hash_ref(slice: &[u8]) -> Digest {
    let mut context = Context::new(&SHA256);
    context.update(slice);
    let hash = context.finish();
    hash
}
