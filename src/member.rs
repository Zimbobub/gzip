use crate::{Parse, flags::Flags, header::Header};
use std::io::Read;





struct CRC {
    pub crc_16: u16,
    pub compressed_blocks: Vec<u8>,
    pub crc_32: u32,
    pub isize: u32
}

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
pub struct Member {
    header: Header,
    extra: Vec<ExtraField>,
    filename: Option<std::ffi::CString>,
    file_comment: Option<std::ffi::CString>,

    fhcrc: Option<CRC>,
}



impl Parse for Member {
    fn read_from_file<R>(buffer: &mut std::io::BufReader<R>) -> Option<Self> where R: std::io::Read, Self: Sized {
        let header = Header::read_from_file(buffer)?;
        
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


        return None;
    }
}

