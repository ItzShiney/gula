use super::Deserialize;
use super::Serialize;

#[macro_export]
macro_rules! impl_bytes_serde {
    ($T:ty) => {
        impl $crate::serde::Serialize for $T
        where
            $T: Copy,
        {
            #[inline(always)]
            fn extend_serialized(&self, out: &mut Vec<u8>) {
                let ne_bytes = unsafe {
                    std::slice::from_raw_parts(
                        self as *const $T as *const u8,
                        std::mem::size_of::<$T>(),
                    )
                };
                out.extend_from_slice(ne_bytes);
            }

            #[inline(always)]
            fn serialized_len(&self) -> usize {
                std::mem::size_of::<$T>()
            }

            #[inline(always)]
            fn serialize(&self) -> Vec<u8> {
                let ne_bytes = unsafe {
                    std::slice::from_raw_parts(
                        self as *const _ as *const u8,
                        std::mem::size_of::<$T>(),
                    )
                };
                ne_bytes.into()
            }
        }

        impl $crate::serde::Deserialize for $T {
            #[inline(always)]
            fn deserialize(bytes: &[u8]) -> $T {
                unsafe { std::ptr::read_unaligned(bytes.as_ptr() as *const $T) }
                /* let bytes = unsafe { bytes.get_unchecked(0..std::mem::size_of::<$T>()) }
                    .try_into()
                    .expect(concat!(
                        "expected size_of<",
                        stringify!($T),
                        ">() bytes for `from_ne_bytes`"
                    ));
                <$T>::from_ne_bytes(bytes) */
            }
        }
    };
}

#[macro_export]
macro_rules! impl_0_serde {
    ($T:ty) => {
        impl $crate::serde::Serialize for $T {
            #[inline(always)]
            fn extend_serialized(&self, out: &mut Vec<u8>) {
                self.0.extend_serialized(out)
            }

            #[inline(always)]
            fn serialized_len(&self) -> usize {
                self.0.serialized_len()
            }

            #[inline(always)]
            fn serialize(&self) -> Vec<u8> {
                self.0.serialize()
            }
        }

        impl $crate::serde::Deserialize for $T {
            #[inline(always)]
            fn deserialize(bytes: &[u8]) -> $T {
                Self(<_>::deserialize(bytes))
            }
        }
    };
}

impl_bytes_serde!(i8);
impl_bytes_serde!(i16);
impl_bytes_serde!(i32);
impl_bytes_serde!(i64);
impl_bytes_serde!(i128);
impl_bytes_serde!(isize);
impl_bytes_serde!(u8);
impl_bytes_serde!(u16);
impl_bytes_serde!(u32);
impl_bytes_serde!(u64);
impl_bytes_serde!(u128);
impl_bytes_serde!(usize);
impl_bytes_serde!(f32);
impl_bytes_serde!(f64);

impl Serialize for bool {
    #[inline(always)]
    fn extend_serialized(&self, out: &mut Vec<u8>) {
        out.push(*self as u8);
    }

    #[inline(always)]
    fn serialized_len(&self) -> usize {
        1
    }

    #[inline(always)]
    fn serialize(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

impl Deserialize for bool {
    #[inline(always)]
    fn deserialize(bytes: &[u8]) -> Self {
        unsafe { *bytes.get_unchecked(0) != 0 }
    }
}
