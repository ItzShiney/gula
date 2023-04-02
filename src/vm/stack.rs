use super::HeapObjectID;
use crate::types::Bool;
use crate::types::Int;

pub trait StackTrait<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> T;
    fn last(&self) -> T;
}

macro_rules! stack {
    (
        $($Name:ident),* $(,)?
    ) => {
        ::paste::paste! {
            #[derive(Debug, Default)]
            pub struct Stack {
                $(
                    [<$Name:snake>]: Vec<$Name>,
                )*
            }

            $(
                impl StackTrait<$Name> for Stack {
                    fn push(&mut self, value: $Name) {
                        self.[<$Name:snake>].push(value);
                    }

                    fn pop(&mut self) -> $Name {
                        self.[<$Name:snake>].pop().unwrap()
                    }

                    fn last(&self) -> $Name {
                        *self.[<$Name:snake>].last().unwrap()
                    }
                }
            )*
        }
    };
}

stack!(Int, Bool, HeapObjectID);
