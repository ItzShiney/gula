#[macro_use]
mod macro_;

use crate::types::Bool;
use crate::types::Int;
use crate::types::Str;
use crate::vm::HeapObjectID;
use crate::vm::StackTrait;

instruction! {
    instructions, vm:

    IntPush(value: Int) = {
        vm.stack.push(value);
    }

    IntPop = |_value: Int| {}

    IntDup = {
        let value: Int = vm.stack.last();
        vm.stack.push(value);
    }

    IntAdd = |a: Int, b: Int| {
        vm.stack.push(a + b);
    }

    IntSub = |a: Int, b: Int| {
        vm.stack.push(a - b);
    }

    IntMod = |a: Int, b: Int| {
        vm.stack.push(a % b);
    }

    IntEq = |a: Int, b: Int| {
        vm.stack.push(a == b);
    }

    IntLe = |a: Int, b: Int| {
        vm.stack.push(a <= b);
    }

    IntPrint = |value: Int| {
        #[allow(unused)] let value = value;

        #[cfg(feature = "io")]
        println!("{}", value);
    }

    HeapObjectIDPush(value: HeapObjectID) = {
        vm.stack.push(value);
    }

    StrPrint = |value: HeapObjectID| {
        #[allow(unused)] let value = vm.heap.get::<Str>(value);

        #[cfg(feature = "io")]
        println!("{}", value);
    }

    Jump(instructions_skipped: isize) = {
        instructions.jump(instructions_skipped);
    }

    JumpUnless(instructions_skipped: isize) = |condition: Bool| {
        if !condition {
            instructions.jump(instructions_skipped);
        }
    }

    JumpIf(instructions_skipped: isize) = |condition: Bool| {
        if condition {
            instructions.jump(instructions_skipped);
        }
    }

    Out = {
        instructions.end();
    }
}
