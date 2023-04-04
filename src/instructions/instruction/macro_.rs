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

        $first
    };
}

#[macro_export]
macro_rules! instruction {
    (
        $instructions:ident, $vm:ident:

        $(
            $Name:ident
                $(
                    (
                        $($bytes_arg:ident : $BytesArg:ty),+ $(,)?
                    )
                )? = $(|
                    $($pop_arg:ident : $PopArg:ty),+ $(,)?
                |)?
            $body:block
        )*
    ) => {::paste::paste! {
        #[derive(Debug, Clone, Copy)]
        #[repr(u16)]
        pub enum Instruction {
            $($Name $( ( $($BytesArg)+ ) )?,)*
        }

        impl Instruction {
            #[inline(always)]
            pub fn eval(self, #[allow(unused)] $instructions: &mut $crate::instructions::Instructions, $vm: &mut $crate::vm::VM) {
                match self {
                    $(
                        Self::$Name $( ( $($bytes_arg)+ ) )? => {
                            $crate::__reverse_statements! {
                                $( $(
                                    let $pop_arg: $PopArg = $vm.stack.pop();
                                )+ )?
                            }

                            $body
                        }
                    )*
                }
            }
        }
    }};
}
