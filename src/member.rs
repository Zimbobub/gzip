

struct Header {
    id1: u8,
    id2: u8,
    pub compression_method: u8,
    pub flags: u8,
    pub modification_time: std::time::SystemTime,
    pub extra_flags: u8,
    pub os: u8
}

struct CRC {
    pub crc_16: u16,
    pub compressed_blocks: Vec<u8>,
    pub crc_32: u32,
    pub isize: u32
}

/// A gzipped file is made up of 1 or more 'members' (compressed data sets)
pub struct Member {
    header: Header,
    extra: Option<Vec<u8>>,
    filename: Option<std::ffi::CString>,
    file_comment: Option<std::ffi::CString>,

    fhcrc: Option<CRC>,
}