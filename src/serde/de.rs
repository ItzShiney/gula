use super::BytewiseSerialized;

pub trait Deserialize<T> {
    fn deserialize(&self) -> T;
}

impl<T: BytewiseSerialized> Deserialize<T> for [u8] {
    fn deserialize(&self) -> T {
        unsafe { *(self as *const _ as *const T) }
    }
}
