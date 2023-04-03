use crate::serde::BytesDeserialize;
use crate::serde::Deserialize;
use crate::serde::Serialize;
use std::mem::size_of;

#[derive(Default)]
pub struct Stack(Vec<u8>);

pub trait StackTrait<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> T;
    fn last(&self) -> T;
}

impl<T: Serialize + Deserialize> StackTrait<T> for Stack {
    #[inline(always)]
    fn push(&mut self, value: T) {
        value.extend_serialized(&mut self.0);
    }

    #[inline(always)]
    fn pop(&mut self) -> T {
        let res = self.last();
        let len = size_of::<T>();
        self.0.truncate(self.0.len() - len);
        res
    }

    #[inline(always)]
    fn last(&self) -> T {
        let len = size_of::<T>();
        self.0[self.0.len() - len..].deserialize()
    }
}
