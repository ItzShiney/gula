pub trait Deserialize {
    fn deserialize(bytes: &[u8]) -> Self;
}
