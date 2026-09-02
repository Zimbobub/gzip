use crate::{Parse, deflate::DeflateBlock, flags::Flags, header::Header};
use std::io::{BufRead, Read};





#[derive(Debug)]
struct CompressedData {
    pub crc_16: Option<u16>,
    pub compressed_blocks: Vec<DeflateBlock>,
    pub crc_32: u32,
    pub isize: u32
}

#[derive(Debug)]
struct ExtraField {
    pub subfield_id_1: u8,
    pub subfield_id_2: u8,
    // 16 bit len put here
    pub data: Vec<u8>
}


impl Parse for ExtraField {
    fn read_from_file<R>(buffer: &mut std::io::BufReader<R>) -> Option<Self> where R: std::io::Read, Self: Sized {
        let mut extra_field_header_bytes: [u8; 4] = [0; 4];
        buffer.read_exact(&mut extra_field_header_bytes).ok()?;

        let len = u16::from_le_bytes([extra_field_header_bytes[2], extra_field_header_bytes[3]]);
        let mut data: Vec<u8> = Vec::with_capacity(len as usize);
        buffer.read_exact(&mut data).ok()?;

        return Some(ExtraField { subfield_id_1: extra_field_header_bytes[0], subfield_id_2: extra_field_header_bytes[1], data });
    }
}


/// A gzipped file is made up of 1 or more 'members' (compressed data sets)
#[derive(Debug)]
pub struct Member {
    header: Header,
    extra: Vec<ExtraField>,
    filename: Option<String>,
    file_comment: Option<String>,

    compressed_data: CompressedData,
}



impl Parse for Member {
    fn read_from_file<R>(buffer: &mut std::io::BufReader<R>) -> Option<Self> where R: std::io::Read, Self: Sized {
        let header = Header::read_from_file(buffer)?;
        
        dbg!(&header);

        // ensure using DEFLATE
        if header.compression_method != 8 { return None; }

        // extra fields
        let mut extra_fields: Vec<ExtraField> = Vec::new();
        if header.flags.extra_fields_present() {
            let mut xlen_bytes: [u8; 2] = [0; 2];
            buffer.read_exact(&mut xlen_bytes).ok()?;
            let xlen: usize = u16::from_le_bytes([xlen_bytes[0], xlen_bytes[1]]) as usize;

            let mut read_bytes: usize = 0;
            loop {
                let extra_subfield = ExtraField::read_from_file(buffer)?;
                
                // subfield header + num bytes read
                read_bytes += 4 + extra_subfield.data.len();

                extra_fields.push(extra_subfield);

                if read_bytes >= xlen { break; }
            }
        }
        dbg!(&extra_fields);

        // filename
        let filename: Option<String> = if header.flags.filename_present() {
            let mut filename_bytes: Vec<u8> = Vec::new();
            buffer.read_until(b'\0', &mut filename_bytes).ok()?;
            // remove null terminator
            if *filename_bytes.last()? != b'\0' { return None; }
            filename_bytes.pop();

            Some(String::from_utf8(filename_bytes).ok()?)
        } else {
            None
        };
        dbg!(&filename);

        // file comment
        let file_comment: Option<String> = if header.flags.file_comment_present() {
            let mut file_comment_bytes: Vec<u8> = Vec::new();
            buffer.read_until(b'\0', &mut file_comment_bytes).ok()?;
            // remove null terminator
            if *file_comment_bytes.last()? != b'\0' { return None; }
            file_comment_bytes.pop();

            Some(String::from_utf8(file_comment_bytes).ok()?)
        } else {
            None
        };
        dbg!(&file_comment);

        // CRC16
        let crc_16: Option<u16> = if header.flags.crc_16_present() {
            let mut buf: [u8; 2] = [0; 2];
            buffer.read_exact(&mut buf).ok()?;
            Some(u16::from_le_bytes([buf[0], buf[1]]))
        } else {
            None
        };
        println!("{:x?}", crc_16);

        // COMPRESSED DATA
        let mut blocks: Vec<DeflateBlock> = Vec::new();
        loop {
            let block = DeflateBlock::read_from_file(buffer)?;
            dbg!(&block);
            if block.is_last_block {
                blocks.push(block);
                break;
            } else {
                blocks.push(block);
            }
        }

        // CRC32 and isize
        let mut buf: [u8; 4] = [0; 4];
        buffer.read_exact(&mut buf).expect("failed to read crc32");
        let crc_32 = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);

        let mut buf: [u8; 4] = [0; 4];
        buffer.read_exact(&mut buf).expect("failed to read isize");
        let uncompressed_data_size = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);

        return Some(Member {
            header,
            extra: extra_fields,
            filename: filename,
            file_comment: file_comment,
            compressed_data: CompressedData {
                crc_16,
                compressed_blocks: blocks,
                crc_32,
                isize: uncompressed_data_size
            }
        });
    }
}

