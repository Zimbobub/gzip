pub mod bitstream;
pub mod huffman;
pub mod lz77;

use std::io::Read;

use crate::{Parse, deflate::bitstream::{BitStream, Bits}};



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
        // std::fs::write("./example.deflate", out).unwrap();
        println!("CURRENT BUFFER BEFORE DEFLATE BLOCK {:X?}", buffer.buffer());

        let mut bits = BitStream::new(buffer);

        let is_last_block: Bits<1> =  bits.read_bits();
        let is_last_block: bool = is_last_block.bit(0);

        let compression_type: Bits<2> =  bits.read_bits();
        let compression_type = match (compression_type.bit(0), compression_type.bit(1)) {
            (false, false) => BlockType::Uncompressed,
            (false, true) => BlockType::CompressedFixedHuffman,
            (true, false) => BlockType::CompressedDynamicHuffman,
            (true, true) => BlockType::Reserved,
        };
        dbg!(&is_last_block, &compression_type);

        let data: Vec<u8> = match compression_type {
            BlockType::Uncompressed => {
                // skip to next byte boundary, so just use the regular bufreader

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
                let hlit: Bits<5> = bits.read_bits();
                let hlit: u8 = hlit.inner()[0];

                let hdist: Bits<5> = bits.read_bits();
                let hdist: u8 = hdist.inner()[0];

                let hclen: Bits<4> = bits.read_bits();
                let hclen: u8 = hclen.inner()[0];

                println!("hlit {} hdist {} hclen {}", hlit, hdist, hclen);
                vec![]
            },
            BlockType::Reserved => unimplemented!()
        };
        
        
        return Some(DeflateBlock { is_last_block, compression_type, data });
    }
}