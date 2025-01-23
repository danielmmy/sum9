use std::hash::Hasher;

#[cfg(target_pointer_width = "64")]
const K: usize = 0xf1357aea2e62a9c5;
#[cfg(target_pointer_width = "32")]
const K: usize = 0x93d765dd;

#[derive(Clone)]
pub struct GoodMultiplierHashI32 {
    hash: usize,
}

impl Default for GoodMultiplierHashI32 {
    #[inline]
    fn default() -> Self {
        GoodMultiplierHashI32 { hash: 0 }
    }
}

impl GoodMultiplierHashI32 {
    #[inline]
    fn add_to_hash(&mut self, i: usize) {
        self.hash = self.hash.wrapping_add(i).wrapping_mul(K);
    }
}

impl Hasher for GoodMultiplierHashI32 {
    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.add_to_hash(i as usize);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash as u64
    }

    #[inline]
    fn write(&mut self, _bytes: &[u8]) {
        panic!("Avaible only for i32")
    }
}
