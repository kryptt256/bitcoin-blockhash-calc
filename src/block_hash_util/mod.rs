extern crate ring;

use ring::digest::{Context, Digest, SHA256};
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, LittleEndian, WriteBytesExt};

pub struct BlockHeader {
    pub version: u32,
    pub hash_prev_block: Vec<u8>,
    pub hash_merkle_root: Vec<u8>,
    pub time: u32,
    pub bits: u32,
    pub nonce: u32
}

pub fn get_block_bytes(block_header: BlockHeader) -> Vec<u8> {
    let header_hash = vec![get_vec_as_u32(block_header.hash_prev_block), get_vec_as_u32(block_header.hash_merkle_root),
                            block_header.time, block_header.bits, block_header.nonce]; 
    
    let mut wtr: Vec<u8> = vec![];
    header_hash.iter().for_each(|number| wtr.write_u32::<LittleEndian>(*number).unwrap());
    wtr
}

fn get_vec_as_u32(bytes: Vec<u8>) -> u32 {
    let mut reader = Cursor::new(bytes);
    reader.read_u32::<BigEndian>().unwrap()
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
