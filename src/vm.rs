mod heap;
mod stack;

pub use heap::*;
pub use stack::*;

#[derive(Default)]
pub struct VM {
    pub stack: Stack,
    pub heap: Heap,
}
