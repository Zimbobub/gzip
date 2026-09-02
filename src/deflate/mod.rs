pub mod bitstream;
pub mod huffman;
pub mod lz77;

use std::io::Read;

use crate::Parse;



#[derive(Debug)]
pub enum BlockType {
    Uncompressed,
    CompressedFixedHuffman,
    CompressedDynamicHuffman,
    Reserved
}

/// All little endian, except huffman codes
#[derive(Debug)]
pub struct DeflateBlock {
    pub is_last_block: bool,
    pub compression_type: BlockType,
    pub data: Vec<u8>
}

impl Parse for DeflateBlock {
    fn read_from_file<R>(buffer: &mut std::io::BufReader<R>) -> Option<Self> where R: std::io::Read, Self: Sized {
        // let mut out: Vec<u8> = Vec::new();
        // buffer.read_to_end(&mut out).unwrap();
        // std::fs::write("./DeflatBlock", out).unwrap();


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

        println!("deflate header {:x}: {:b}", header_byte[0], header_byte[0]);

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
            BlockType::CompressedFixedHuffman => unimplemented!(),
            BlockType::CompressedDynamicHuffman => {
                vec![]
            },
            BlockType::Reserved => unimplemented!()
        };
        
        
        return Some(DeflateBlock { is_last_block, compression_type, data });
    }
}