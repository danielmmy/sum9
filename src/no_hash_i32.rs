use std::{hash::Hasher, marker::PhantomData};

#[derive(Debug, Copy)]
pub struct NoHashI32(i32, PhantomData<i32>);

impl Default for NoHashI32 {
    fn default() -> Self {
        NoHashI32(0, PhantomData)
    }
}

impl Clone for NoHashI32 {
    fn clone(&self) -> Self {
        *self
    }
}

impl Hasher for NoHashI32 {
    fn write_i32(&mut self, i: i32) {
        self.0 = i;
    }

    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!("Avaible only for i32")
    }
}
