use std::{collections::VecDeque, fmt::Debug, io::Read};


pub struct Bits<const N: usize> {
    inner: Vec<u8>
}

impl<const N: usize> Bits<N> {
    pub fn new() -> Bits<N> {
        return Bits { inner: vec![0; N.div_ceil(8)] };
    }

    pub fn inner(&self) -> &Vec<u8> {
        return &self.inner;
    }

    pub fn bit(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let byte = self.inner[byte_index];
        return (byte >> (index % 8)) == 1;
    }

    pub fn set_bit(&mut self, index: usize, value: bool) {    
        // set bit to 0
        self.inner[index / 8] &= !(1 << (index % 8));
        // then or with `value`
        self.inner[index / 8] |= (value as u8) << (index % 8);
    }
}

impl<const N: usize> Into<[bool; N]> for &Bits<N> {
    fn into(self) -> [bool; N] {
        let mut out = [false; N];

        for i in 0..N {
            out[i] = self.bit(i);
        }

        return out;
    }
}

impl<const N: usize> Debug for Bits<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bits = Into::<[bool; N]>::into(self);

        let bits = bits.map(|bit| if bit {'1' as u8} else {'0' as u8} );
        let (bytes, remainder) = bits.as_chunks::<8>();

        let bytes: Vec<String> = bytes.iter().map(|b| String::from_utf8(b.to_vec()).unwrap()).collect();
        let remainder = String::from_utf8(remainder.to_vec()).unwrap();
        
        f.debug_list().entries(bytes).entry(&remainder).finish()
    }
}


/// Note: will always read full bytes, so <R> will always be on a byte boundary
pub struct BitStream<'a, R: Read> {
    inner: &'a mut R,
    // bits from a half-consumed byte
    bit_buf: VecDeque<bool>
}

impl<'a, R: Read> BitStream<'a, R> {
    pub fn new(read_buffer: &'a mut R) -> BitStream<'a, R> {
        return BitStream {
            inner: read_buffer,
            bit_buf: VecDeque::with_capacity(8)
        }
    }

    pub fn into_inner(&mut self) -> &mut R {
        return self.inner;
    }

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
        // println!("reading {N} bytes {:?}", self.bit_buf);

        let mut out: Bits<N> = Bits::new();

        for i in 0..N {
            let next_bit = self.next().expect("REACHED END OF STREAM");
            out.set_bit(i, next_bit);
        }

        // dbg!(&out);
        
        return out;
    }
}
