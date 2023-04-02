#[macro_export]
macro_rules! __reverse_statements {
    () => {};

    (
        $first:stmt;
        $($others:stmt;)*
    ) => {
        $crate::__reverse_statements! {
            $($others;)*
        }

        $first;
    };
}

#[macro_export]
macro_rules! instruction {
    (
        @discriminant [$discriminant:expr]
        @enum_cases [$($enum_cases:tt)*]
        @instructions [$($output_instructions:tt)*]
        @discriminant_match [$($discriminant_match:tt)*]
        @serialize_match [$($serialize_match:tt)*]
        @serialized_len_match [$($serialized_len_match:tt)*]
        @out [$out:ident]
        @self [$self:ident]
        @deserialize_match [$($deserialize_match:tt)*]

        $Name:ident
            $(
                (
                    $($bytes_arg:ident : $BytesArg:ty),+ $(,)?
                )
            )? = |$instructions:ident, $vm:ident $(
                -> (
                    $($pop_arg:ident : $PopArg:ty),* $(,)?
                )
            )?|
        $body:block

        $($xs:tt)*
    ) => {::paste::paste! {
        const [<$Name:snake:upper>]: $crate::instructions::InstructionID = $discriminant;

        $crate::instruction! {
            @discriminant [ $discriminant + 1 ]
            @enum_cases [
                $($enum_cases)*

                $Name $( ($($BytesArg),+) )? = [<$Name:snake:upper>],
            ]
            @instructions [
                $($output_instructions)*
                |$instructions: &mut $crate::instructions::Instructions, $vm: &mut $crate::vm::VM| -> isize {
                    #[allow(unused_mut)]
                    let mut skip = 0_isize;

                    $($(
                        let $bytes_arg = $instructions.read::<$BytesArg>();
                        skip += std::mem::size_of::<$BytesArg>() as isize;
                    )+)?

                    use $crate::vm::StackTrait;
                    $(
                        $crate::__reverse_statements! {
                            $(
                                let $pop_arg: $PopArg = $vm.stack.pop();
                            )*
                        }
                    )?

                    $body

                    skip
                },
            ]
            @discriminant_match [
                $($discriminant_match)*
                #[allow(unused)] Self::$Name $( ($($bytes_arg),*) )? => [<$Name:snake:upper>],
            ]
            @serialize_match [
                $($serialize_match)*
                #[allow(unused)] Self::$Name $( ($($bytes_arg),*) )? => {
                    $( $(
                        $bytes_arg.extend_serialized($out);
                    )+ )?
                },
            ]
            @serialized_len_match [
                $($serialized_len_match)*
                #[allow(unused)] Self::$Name $( ($($bytes_arg),*) )? => {
                    #[allow(unused_mut)]
                    let mut res = 0_usize;
                    $( $(
                        res += $bytes_arg.serialized_len();
                    )+ )?
                    res
                },
            ]
            @out [$out]
            @self [$self]
            @deserialize_match [
                $($deserialize_match)*
                #[allow(unused)]
                [<$Name:snake:upper>] => {
                    let mut offset = std::mem::size_of::<$crate::instructions::InstructionID>();

                    $( $(
                        let $bytes_arg: $BytesArg = ($self[offset..]).deserialize();
                        offset += $crate::serde::Serialize::serialized_len(&$bytes_arg);
                    )+ )?

                    Instruction::$Name $( ($($bytes_arg,)+) )?
                }
            ]

            $($xs)*
        }
    }};

    (
        @discriminant [$discriminant:expr]
        @enum_cases [$($enum_cases:tt)*]
        @instructions [$($output_instructions:tt)*]
        @discriminant_match [$($discriminant_match:tt)*]
        @serialize_match [$($serialize_match:tt)*]
        @serialized_len_match [$($serialized_len_match:tt)*]
        @out [$out:ident]
        @self [$self:ident]
        @deserialize_match [$($deserialize_match:tt)*]
    ) => {
        #[allow(unused)]
        pub const INSTRUCTIONS: &[fn(&mut $crate::instructions::Instructions, &mut $crate::VM) -> isize] = &[
            $($output_instructions)*
        ];

        #[derive(Debug, Clone, Copy)]
        #[repr(u8)]
        pub enum Instruction {
            $($enum_cases)*
        }

        impl crate::serde::Serialize for Instruction {
            fn extend_serialized(&self, $out: &mut Vec<u8>) {
                let discriminant = match self {
                    $($discriminant_match)*
                };

                discriminant.extend_serialized($out);

                match self {
                    $($serialize_match)*
                }
            }

            fn serialized_len(&self) -> usize {
                std::mem::size_of::<InstructionID>() + match self {
                    $($serialized_len_match)*
                }
            }
        }

        impl $crate::serde::Deserialize<Instruction> for [u8] {
            fn deserialize(&$self) -> Instruction {
                match $crate::serde::Deserialize::<$crate::instructions::InstructionID>::deserialize($self) {
                    $($deserialize_match)*
                    _ => panic!("invalid instruction code"),
                }
            }
        }
    };

    (
        @discriminant [$discriminant:expr]
        @enum_cases [$($enum_cases:tt)*]
        @instructions [$($output_instructions:tt)*]
        @discriminant_match [$($discriminant_match:tt)*]
        @serialize_match [$($serialize_match:tt)*]
        @serialized_len_match [$($serialized_len_match:tt)*]
        @out [$out:ident]
        @self [$self:ident]
        @deserialize_match [$($deserialize_match:tt)*]

        $($xs:tt)*
    ) => {
        compile_error!(concat!("could not parse '", stringify!($($xs)*), "'"));
    };

    (
        $($xs:tt)*
    ) => {
        $crate::instruction! {
            @discriminant [0]
            @enum_cases []
            @instructions []
            @discriminant_match []
            @serialize_match []
            @serialized_len_match []
            @out [out]
            @self [self]
            @deserialize_match []

            $($xs)*
        }
    };
}
