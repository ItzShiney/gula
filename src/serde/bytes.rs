use super::Deserialize;

pub trait BytesDeserialize<T> {
    fn deserialize(&self) -> T;
}

impl<T: Deserialize> BytesDeserialize<T> for [u8] {
    #[inline(always)]
    fn deserialize(&self) -> T {
        T::deserialize(self)
    }
}
