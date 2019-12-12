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
    concat_block_header(block_header.version, prev_block, merkle_root, block_header.time, block_header.bits, block_header.nonce)
}

fn concat_block_header(version: u32, prev_block: Vec<u8>, merkle_root: Vec<u8>, time: u32, bits: u32, nonce: u32) -> Vec<u8> {
    vec![(version.to_le_bytes()).to_vec(), prev_block, merkle_root, time.to_le_bytes().to_vec(), 
    bits.to_le_bytes().to_vec(), nonce.to_le_bytes().to_vec()].iter().flatten().cloned().collect()
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
