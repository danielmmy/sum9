use std::{hash::Hasher, marker::PhantomData};

#[derive(Debug, Copy)]
pub struct NoHash<T>(u64, PhantomData<T>);

impl<T> Default for NoHash<T> {
    fn default() -> Self {
        NoHash(0, PhantomData)
    }
}

impl<T> Clone for NoHash<T> {
    fn clone(&self) -> Self {
        NoHash(self.0, self.1)
    }
}

impl<T> Hasher for NoHash<T> {
    fn write(&mut self, bytes: &[u8]) {
        for i in bytes {
            self.write_u8(*i);
        }
    }

    fn write_u8(&mut self, i: u8) {
        self.0 = u64::from(i)
    }
    fn write_u16(&mut self, i: u16) {
        self.0 = u64::from(i)
    }
    fn write_u32(&mut self, i: u32) {
        self.0 = u64::from(i)
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = i
    }
    fn write_usize(&mut self, i: usize) {
        self.0 = i as u64
    }
    fn write_i8(&mut self, i: i8) {
        self.0 = i as u64
    }
    fn write_i16(&mut self, i: i16) {
        self.0 = i as u64
    }
    fn write_i32(&mut self, i: i32) {
        self.0 = i as u64
    }
    fn write_i64(&mut self, i: i64) {
        self.0 = i as u64
    }
    fn write_isize(&mut self, i: isize) {
        self.0 = i as u64
    }

    fn finish(&self) -> u64 {
        self.0
    }
}
