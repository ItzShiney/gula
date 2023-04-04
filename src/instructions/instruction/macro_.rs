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
        @deserialize_arg [$deserialize_arg:ident]
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

                #[inline(always)]
                |$instructions: &mut $crate::instructions::Instructions, $vm: &mut $crate::vm::VM| {
                    $($(
                        let $bytes_arg = $instructions.read::<$BytesArg>();
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
            @deserialize_arg [$deserialize_arg]
            @deserialize_match [
                $($deserialize_match)*
                #[allow(unused)]
                [<$Name:snake:upper>] => {
                    let mut offset = std::mem::size_of::<$crate::instructions::InstructionID>();

                    $( $(
                        let $bytes_arg: $BytesArg = $crate::serde::BytesDeserialize::deserialize(&$deserialize_arg[offset..]);
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
        @deserialize_arg [$deserialize_arg:ident]
        @deserialize_match [$($deserialize_match:tt)*]
    ) => {
        #[derive(Debug, Clone, Copy)]
        #[repr(u16)]
        pub enum Instruction {
            $($enum_cases)*
        }

        impl Instruction {
            #[inline(always)]
            pub fn eval(instruction_id: $crate::instructions::InstructionID, instructions: &mut $crate::instructions::Instructions, vm: &mut $crate::vm::VM) {
                #[allow(unused)]
                const INSTRUCTIONS: &[fn(&mut $crate::instructions::Instructions, &mut $crate::VM)] = &[
                    $($output_instructions)*
                ];

                unsafe {
                    INSTRUCTIONS.get_unchecked(instruction_id as usize)(instructions, vm);
                }
            }
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

            fn serialize(&self) -> Vec<u8> {
                let mut res = Vec::default();
                self.extend_serialized(&mut res);
                res
            }
        }

        impl $crate::serde::Deserialize for Instruction {
            fn deserialize($deserialize_arg: &[u8]) -> Instruction {
                match $crate::instructions::InstructionID::deserialize($deserialize_arg) {
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
        @deserialize_arg [$deserialize_arg:ident]
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
            @deserialize_arg [value]
            @deserialize_match []

            $($xs)*
        }
    };
}
