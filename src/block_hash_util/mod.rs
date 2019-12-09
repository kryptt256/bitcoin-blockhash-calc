extern crate hmac_sha256;

use hmac_sha256::Hash;
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

