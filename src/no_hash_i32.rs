use std::hash::Hasher;

#[cfg(target_pointer_width = "64")]
const K: usize = 0xf1357aea2e62a9c5;
#[cfg(target_pointer_width = "32")]
const K: usize = 0x93d765dd;

#[derive(Clone)]
pub struct NoHashI32(usize);

impl Default for NoHashI32 {
    #[inline]
    fn default() -> Self {
        NoHashI32(0)
    }
}

impl NoHashI32 {
    #[inline]
    fn add_to_hash(&mut self, i: usize) {
        self.0 = self.0.wrapping_add(i).wrapping_mul(K);
    }
}

impl Hasher for NoHashI32 {
    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.add_to_hash(i as usize);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    #[inline]
    fn write(&mut self, _bytes: &[u8]) {
        panic!("Avaible only for i32")
    }
}
