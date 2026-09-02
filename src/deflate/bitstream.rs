use std::{collections::VecDeque, io::Read};


pub struct Bits<const N: usize> {
    inner: Vec<u8>
}

impl<const N: usize> Bits<N> {
    pub fn new() -> Bits<N> {
        return Bits { inner: vec![0; N.div_ceil(8)] };
    }

    pub fn bit(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let byte = self.inner[byte_index];
        return (byte >> (index % 8)) == 1;
    }

    pub fn set_bit(&mut self, index: usize, value: bool) {    
        // set bit to 0
        self.inner[index / 8] &= (0b1111_1110) << (index % 8);
        // then or with `value`
        self.inner[index / 8] |= (value as u8) << (index % 8);
    }
}


/// Note: will always read full bytes, so <R> will always be on a byte boundary
pub struct BitStream<R: Read> {
    inner: R,
    // bits from a half-consumed byte
    bit_buf: VecDeque<bool>
}

impl<R: Read> BitStream<R> {
    fn next(&mut self) -> Option<bool> {
        if let Some(bit) = self.bit_buf.pop_front() {
            return Some(bit);
        } else {
            // push next byte into bit buf
            let mut byte: [u8; 1] = [0; 1];
            self.inner.read_exact(&mut byte).ok()?;

            // make most significant bit be pushed to front of the queue, so they are the right way around
            let mut byte: u8 = byte[0].reverse_bits();
            for _ in 0..8 {
                self.bit_buf.push_back((byte & 0x1) == 1);
                byte >>= 1;
            }
            
            return self.bit_buf.pop_front();
        }
    }

    pub fn read_bits<const N: usize>(&mut self) -> Bits<N> {
        let mut out: Bits<N> = Bits::new();

        for i in 0..N {
            out.set_bit(i, self.next().expect("REACHED END OF STREAM"));
        }
        
        return out;
    }
}
