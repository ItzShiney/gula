use crate::impl_0_serde;

#[derive(Debug, Clone, Copy)]
pub struct HeapObjectID(pub(super) usize);

impl_0_serde!(HeapObjectID);
