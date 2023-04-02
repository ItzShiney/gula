use crate::serde::BytewiseSerialized;
use crate::types::Str;
use std::any::Any;

pub trait HeapPushable {}

impl HeapPushable for Str {}

#[derive(Debug, Clone, Copy)]
pub struct HeapObjectID(usize);
impl BytewiseSerialized for HeapObjectID {}

#[derive(Default)]
pub struct Heap(Vec<Box<dyn Any>>);

impl Heap {
    pub fn push<T: HeapPushable + 'static>(&mut self, object: T) -> HeapObjectID {
        self.0.push(Box::new(object));
        HeapObjectID(self.0.len() - 1)
    }

    pub fn get<T: HeapPushable + Any>(&self, id: HeapObjectID) -> &T {
        let Some(res) = self.0[id.0].downcast_ref() else {
            panic!("expected heap object to be {}", std::any::type_name::<T>());
        };

        res
    }
}
