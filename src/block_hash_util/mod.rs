
pub struct BlockHeader {
    pub version: u32,
    pub hash_prev_block: Vec<u8>,
    pub hash_merkle_root: Vec<u8>,
    pub time: u32,
    pub bits: u32,
    pub nonce: u32
}

use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, LittleEndian, WriteBytesExt};

pub fn get_block_bytes(block_header: BlockHeader) -> Vec<u8> {
    let mut rprev_block = Cursor::new(block_header.hash_prev_block);
    let mut rmerkle_root = Cursor::new(block_header.hash_merkle_root);
    let prev_block = rprev_block.read_u32::<BigEndian>().unwrap();
    let merkle_root = rmerkle_root.read_u32::<BigEndian>().unwrap();

    let mut wtr: Vec<u8> = vec![];
    wtr.write_u32::<LittleEndian>(block_header.version).unwrap();
    wtr.write_u32::<LittleEndian>(prev_block).unwrap();
    wtr.write_u32::<LittleEndian>(merkle_root).unwrap();
    wtr.write_u32::<LittleEndian>(block_header.time).unwrap();
    wtr.write_u32::<LittleEndian>(block_header.bits).unwrap();
    wtr.write_u32::<LittleEndian>(block_header.nonce).unwrap();
    wtr
}