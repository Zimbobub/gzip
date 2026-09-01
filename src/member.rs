use crate::header::Header;





struct CRC {
    pub crc_16: u16,
    pub compressed_blocks: Vec<u8>,
    pub crc_32: u32,
    pub isize: u32
}

struct ExtraField {
    pub subfield_id: u16,
    // 16 bit len put here
    pub data: Vec<u8>
}

/// A gzipped file is made up of 1 or more 'members' (compressed data sets)
pub struct Member {
    header: Header,
    extra: Option<Vec<u8>>,
    filename: Option<std::ffi::CString>,
    file_comment: Option<std::ffi::CString>,

    fhcrc: Option<CRC>,
}