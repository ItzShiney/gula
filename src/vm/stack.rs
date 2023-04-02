use crate::serde::BytewiseSerialized;
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

impl<T: BytewiseSerialized> StackTrait<T> for Stack {
    fn push(&mut self, value: T) {
        value.extend_serialized(&mut self.0);
    }

    fn pop(&mut self) -> T {
        let res = self.last();
        let len = size_of::<T>();
        self.0.truncate(self.0.len() - len);
        res
    }

    fn last(&self) -> T {
        let len = size_of::<T>();
        self.0[self.0.len() - len..].deserialize()
    }
}
