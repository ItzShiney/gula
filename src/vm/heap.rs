mod id;

use crate::types::Str;
pub use id::*;
use std::any::type_name;
use std::any::Any;

pub trait HeapPushable {}

impl HeapPushable for Str {}

#[derive(Default)]
pub struct Heap(Vec<Box<dyn Any>>);

impl Heap {
    pub fn push<T: HeapPushable + Any>(&mut self, object: T) -> HeapObjectID {
        self.0.push(Box::new(object));
        HeapObjectID(self.0.len() - 1)
    }

    pub fn get<T: HeapPushable + Any>(&self, id: HeapObjectID) -> &T {
        let Some(res) = self.0[id.0].downcast_ref() else {
            panic!("expected heap object to be {}", type_name::<T>());
        };

        res
    }
}
