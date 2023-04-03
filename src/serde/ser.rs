pub trait Serialize: Sized {
    fn extend_serialized(&self, out: &mut Vec<u8>);
    fn serialized_len(&self) -> usize;
    fn serialize(&self) -> Vec<u8>;
}
