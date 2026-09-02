


/// bit 0 FTEXT
/// bit 1 FHCRC
/// bit 2 FEXTRA
/// bit 3 FNAME
/// bit 4 FCOMMENT
/// bit 5 reserved
/// bit 6 reserved
/// bit 7 reserved
#[derive(Debug)]
pub struct Flags(pub u8);

impl Flags {
    /// Just an indicator, can be ignored
    pub fn data_is_ascii(&self) -> bool {
        return (self.0 & 0x1) == 0x1;
    }

    /// CRC16 put right before the compressed data
    pub fn crc_16_present(&self) -> bool {
        return (self.0 & 0x2) == 0x2;
    }

    /// See: `ExtraField`
    pub fn extra_fields_present(&self) -> bool {
        return (self.0 & 0x4) == 0x4;
    }

    pub fn filename_present(&self) -> bool {
        return (self.0 & 0x8) == 0x8;
    }

    pub fn file_comment_present(&self) -> bool {
        return (self.0 & 0x10) == 0x10;
    }
}


