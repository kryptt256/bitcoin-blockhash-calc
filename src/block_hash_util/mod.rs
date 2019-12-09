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

