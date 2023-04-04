mod instruction;

use crate::vm::VM;
pub use instruction::*;
use std::fmt;
use std::fmt::Debug;

pub struct Instructions {
    instructions: Vec<Instruction>,
    instruction_idx: usize,
}

impl Debug for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "instructions:")?;

        const INSTRUCTIONS_PRINTED_LIMIT: usize = 5;

        for instruction in self.as_slice().iter().take(INSTRUCTIONS_PRINTED_LIMIT) {
            writeln!(f, "    {:?}", instruction)?;
        }

        if self.as_slice().len() > INSTRUCTIONS_PRINTED_LIMIT {
            writeln!(f, "    ...")?;
        }

        Ok(())
    }
}

impl From<Vec<Instruction>> for Instructions {
    fn from(instructions: Vec<Instruction>) -> Self {
        Self { instructions, instruction_idx: 0 }
    }
}

impl Instructions {
    #[inline]
    pub fn as_slice(&self) -> &[Instruction] {
        unsafe { &self.instructions.get_unchecked(self.instruction_idx..) }
    }

    #[inline]
    pub fn current(&self) -> Instruction {
        unsafe { *self.instructions.get_unchecked(self.instruction_idx) }
    }

    #[inline]
    pub fn jump(&mut self, mut instructions_offset: isize) {
        if instructions_offset < 0 {
            instructions_offset -= 1;
        }

        self.instruction_idx =
            self.instruction_idx.checked_add_signed(instructions_offset).unwrap();
    }

    #[inline]
    pub fn end(&mut self) {
        self.instructions.clear();
        self.instruction_idx = 0;
    }

    #[inline]
    pub fn eval(&mut self, vm: &mut VM) {
        while !self.as_slice().is_empty() {
            self.eval_next(vm);
        }
    }

    #[inline]
    pub fn eval_next(&mut self, vm: &mut VM) {
        let instruction = self.current();
        self.instruction_idx += 1;

        instruction.eval(self, vm);
    }
}
