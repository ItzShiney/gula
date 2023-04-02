#![allow(clippy::uninlined_format_args)]
pub mod instructions;
pub mod serde;
pub mod types;
pub mod vm;

use crate::instructions::Instruction;
use crate::instructions::Instructions;
use std::time::Instant;
use vm::VM;

fn main() {
    let mut vm = VM::default();

    let fizzbuzz_str = vm.heap.push("fizzbuzz".to_string());
    let fizz_str = vm.heap.push("fizz".to_string());
    let buzz_str = vm.heap.push("buzz".to_string());

    use Instruction::*;

    #[rustfmt::skip]
    let mut instructions = Instructions::from(vec![
        IntPush(1),
        
        IntDup,
        IntPush(15),
        IntMod,
        IntPush(0),
        IntEq,
        JumpUnless(3),
            HeapObjectIDPush(fizzbuzz_str),
            StrPrint,
            Jump(9 * 2 + 2),

        IntDup,
        IntPush(3),
        IntMod,
        IntPush(0),
        IntEq,
        JumpUnless(3),
            HeapObjectIDPush(fizz_str),
            StrPrint,
            Jump(9 + 2),

        IntDup,
        IntPush(5),
        IntMod,
        IntPush(0),
        IntEq,
        JumpUnless(3),
            HeapObjectIDPush(buzz_str),
            StrPrint,
            Jump(2),

        IntDup,
        IntPrint,

        IntPush(1),
        IntAdd,

        IntDup,
        IntPush(10_000_000),
        IntLe,

        JumpIf(-(3 + 2 + 2 + 9 * 3)),
    ]);

    let instant = Instant::now();
    instructions.eval(&mut vm);
    println!("{:?}", instant.elapsed());
}
