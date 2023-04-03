#[macro_use]
mod macro_;

use crate::types::Bool;
use crate::types::Int;
use crate::types::Str;
use crate::vm::HeapObjectID;

pub type InstructionID = u8;

instruction! {
    IntPush(value: Int) = |instructions, vm| {
        vm.stack.push(value);
    }

    IntPop = |instructions, vm -> (value: Int)| {}

    IntDup = |instructions, vm| {
        let value: Int = vm.stack.last();
        vm.stack.push(value);
    }

    IntAdd = |instructions, vm -> (a: Int, b: Int)| {
        vm.stack.push(a + b);
    }

    IntSub = |instructions, vm -> (a: Int, b: Int)| {
        vm.stack.push(a - b);
    }

    IntMod = |instructions, vm -> (a: Int, b: Int)| {
        vm.stack.push(a % b);
    }

    IntEq = |instructions, vm -> (a: Int, b: Int)| {
        vm.stack.push(a == b);
    }

    IntLe = |instructions, vm -> (a: Int, b: Int)| {
        vm.stack.push(a <= b);
    }

    IntPrint = |instructions, vm -> (value: Int)| {
        // println!("{}", value);
    }

    HeapObjectIDPush(value: HeapObjectID) = |instructions, vm| {
        vm.stack.push(value);
    }

    StrPrint = |instructions, vm -> (value: HeapObjectID)| {
        let value = vm.heap.get::<Str>(value);
        // println!("{}", value);
    }

    Jump(instructions_skipped: isize) = |instructions, stack| {
        /// Instruction::from(Vec<Instruction>) modifies this instruction for `instructions_skipped` to be a valid byte offset
        instructions.jump(instructions_skipped);
    }

    JumpUnless(instructions_skipped: isize) = |instructions, stack -> (condition: Bool)| {
        /// Instruction::from(Vec<Instruction>) modifies this instruction for `instructions_skipped` to be a valid byte offset
        if !condition {
            instructions.jump(instructions_skipped);
        }
    }

    JumpIf(instructions_skipped: isize) = |instructions, stack -> (condition: Bool)| {
        /// Instruction::from(Vec<Instruction>) modifies this instruction for `instructions_skipped` to be a valid byte offset
        if condition {
            instructions.jump(instructions_skipped);
        }
    }

    Out = |instructions, stack| {
        instructions.end();
    }
}
