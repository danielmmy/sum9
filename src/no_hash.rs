use std::hash::Hasher;

const MAX_BYTES: usize = 8;

#[derive(Clone)]
pub struct NoHash(usize);

impl Default for NoHash {
    #[inline]
    fn default() -> Self {
        NoHash(0)
    }
}

impl Hasher for NoHash {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        if bytes.len() > MAX_BYTES {
            panic!("Key must have maximum of {MAX_BYTES} bytes")
        }
        for i in bytes {
            self.write_u8(*i);
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.0 = i as usize;
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.0 = i as usize;
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.0 = i as usize;
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = i as usize;
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.0 = i as usize;
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.0 = i;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0 as u64
    }
}
