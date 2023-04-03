#[macro_use]
mod macro_;

use crate::types::Bool;
use crate::types::Int;
use crate::types::Str;
use crate::vm::HeapObjectID;

pub type InstructionID = u8;

instruction! {
    instructions, vm:

    IntPush(value: Int) {
        vm.stack.push(value);
    }

    IntPop |_value: Int| {}

    IntDup {
        let value: Int = vm.stack.last();
        vm.stack.push(value);
    }

    IntAdd |a: Int, b: Int| {
        vm.stack.push(a + b);
    }

    IntSub |a: Int, b: Int| {
        vm.stack.push(a - b);
    }

    IntMod |a: Int, b: Int| {
        vm.stack.push(a % b);
    }

    IntEq |a: Int, b: Int| {
        vm.stack.push(a == b);
    }

    IntLe |a: Int, b: Int| {
        vm.stack.push(a <= b);
    }

    IntPrint |value: Int| {
        #[allow(unused)] let value = value;
        // println!("{}", value);
    }

    HeapObjectIDPush(value: HeapObjectID) {
        vm.stack.push(value);
    }

    StrPrint |value: HeapObjectID| {
        #[allow(unused)] let value = vm.heap.get::<Str>(value);
        // println!("{}", value);
    }

    Jump(instructions_skipped: isize) {
        // Instruction::from(Vec<Instruction>) modifies this instruction for `instructions_skipped` to be a valid byte offset
        instructions.jump(instructions_skipped);
    }

    JumpUnless(instructions_skipped: isize) |condition: Bool| {
        // Instruction::from(Vec<Instruction>) modifies this instruction for `instructions_skipped` to be a valid byte offset
        if !condition {
            instructions.jump(instructions_skipped);
        }
    }

    JumpIf(instructions_skipped: isize) |condition: Bool| {
        // Instruction::from(Vec<Instruction>) modifies this instruction for `instructions_skipped` to be a valid byte offset
        if condition {
            instructions.jump(instructions_skipped);
        }
    }

    Out {
        instructions.end();
    }
}
