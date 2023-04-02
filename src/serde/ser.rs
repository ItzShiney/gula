use std::mem::size_of;
use std::slice;

pub trait Serialize: Sized {
    fn extend_serialized(&self, out: &mut Vec<u8>);
    fn serialized_len(&self) -> usize;

    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::default();
        self.extend_serialized(&mut res);
        res
    }
}

pub trait BytewiseSerialized: Copy {}

impl BytewiseSerialized for i8 {}
impl BytewiseSerialized for i16 {}
impl BytewiseSerialized for i32 {}
impl BytewiseSerialized for i64 {}
impl BytewiseSerialized for i128 {}
impl BytewiseSerialized for isize {}
impl BytewiseSerialized for u8 {}
impl BytewiseSerialized for u16 {}
impl BytewiseSerialized for u32 {}
impl BytewiseSerialized for u64 {}
impl BytewiseSerialized for u128 {}
impl BytewiseSerialized for usize {}
impl BytewiseSerialized for f32 {}
impl BytewiseSerialized for f64 {}
impl BytewiseSerialized for bool {}

impl<T: BytewiseSerialized> Serialize for T {
    fn extend_serialized(&self, out: &mut Vec<u8>) {
        unsafe {
            let data = self as *const _ as *const u8;
            let len = self.serialized_len();
            out.extend(slice::from_raw_parts(data, len));
        }
    }

    fn serialized_len(&self) -> usize {
        size_of::<T>()
    }
}
