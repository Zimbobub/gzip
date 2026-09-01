use crate::{Parse, flags::Flags};
use std::io::Read;


pub struct Header {
    pub compression_method: u8,
    pub flags: Flags,
    pub modification_time: std::time::SystemTime,
    pub extra_flags: u8,
    pub os: u8
}

impl Parse for Header {
    fn read_from_file<R>(buffer: &mut std::io::BufReader<R>) -> Option<Self> where R: std::io::Read, Self: Sized {
        let mut header_bytes: [u8; 10] = [0; 10];
        buffer.read_exact(&mut header_bytes).ok()?;

        if header_bytes[0] != 0x1f || header_bytes[1] != 0x8b {
            eprintln!("Not a gzip file");
            return None;
        }

        return Some(Header {
            compression_method: header_bytes[2],
            flags: Flags(header_bytes[3]),
            modification_time: std::time::UNIX_EPOCH + std::time::Duration::from_secs(u32::from_le_bytes([header_bytes[4], header_bytes[5], header_bytes[6], header_bytes[7]]) as u64),
            extra_flags: header_bytes[8],
            os: header_bytes[9]
        });
    }
}
