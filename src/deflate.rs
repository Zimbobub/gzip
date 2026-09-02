use std::io::Read;

use crate::Parse;



pub enum BlockType {
    Uncompressed,
    CompressedFixedHuffman,
    CompressedDynamicHuffman,
    Reserved
}

/// All little endian, except huffman codes
pub struct DeflateBlock {
    pub is_last_block: bool,
    pub compression_type: BlockType,
    pub data: Vec<u8>
}

impl Parse for DeflateBlock {
    fn read_from_file<R>(buffer: &mut std::io::BufReader<R>) -> Option<Self> where R: std::io::Read, Self: Sized {
        // only use first 3 bits
        let mut header_byte: [u8; 1] = [0; 1];
        buffer.read_exact(&mut header_byte).ok()?;
        let is_last_block: bool = header_byte[0] & 0x80 != 0;
        let compression_type = match header_byte[0] & 0x60 {
            0x00 => BlockType::Uncompressed,
            0x20 => BlockType::CompressedFixedHuffman,
            0x40 => BlockType::CompressedDynamicHuffman,
            0x60 => BlockType::Reserved,
            _ => unreachable!()
        };

        let data: Vec<u8> = match compression_type {
            BlockType::Uncompressed => {
                let mut uncompressed_block_metadata: [u8; 4] = [0; 4];
                buffer.read_exact(&mut uncompressed_block_metadata).ok()?;
                let len = u16::from_le_bytes([uncompressed_block_metadata[0], uncompressed_block_metadata[1]]);
                let nlen = u16::from_le_bytes([uncompressed_block_metadata[2], uncompressed_block_metadata[3]]);
                // nlen should be the ones complement (bitwise not) of len
                assert_eq!(len, !nlen);

                let mut result: Vec<u8> = Vec::with_capacity(len as usize);
                buffer.read_exact(&mut result).ok()?;
                result
            },
            _ => unimplemented!()
        };
        
        
        return Some(DeflateBlock { is_last_block, compression_type, data });
    }
}